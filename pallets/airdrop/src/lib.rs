#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// All the types and alises must be defined here
mod types;

#[frame_support::pallet]
pub mod pallet {
	use super::types;

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	use fp_evm::LinearCostPrecompile;
	use pallet_evm_precompile_sha3fips::Sha3FIPS256;
	use pallet_evm_precompile_simple::ECRecoverPublicKey;

	use frame_support::fail;
	use frame_support::traits::{Currency, Hooks, ReservableCurrency};
	use frame_system::offchain::CreateSignedTransaction;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// The overarching dispatch call type.
		// type Call: From<Call<Self>>;

		/// The identifier type for an offchain worker.
		type AuthorityId: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>;

		/// Endpoint on where to send request url
		#[pallet::constant]
		type FetchIconEndpoint: Get<&'static str>;
	}

	#[pallet::storage]
	pub(super) type SudoAccount<T: Config> = StorageValue<_, types::AccountIdOf<T>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub sudo_account: types::AccountIdOf<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				sudo_account: types::AccountIdOf::<T>::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<SudoAccount<T>>::set(self.sudo_account.clone());
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// This input have not been added to ice->snapshotmap
		/// because the map already contains the key of this ice_address
		SkippedAddingToMap(types::AccountIdOf<T>),

		/// This input have not been added to pending queue
		/// because same ice_address is already present in queue
		SkippedAddingToQueue(types::AccountIdOf<T>),

		/// This ice address have been added to ice->snapshot map
		/// with default snapshot and provided icon_address
		AddedToMap(types::AccountIdOf<T>),

		/// This request have been sucessfully added to pending queue
		AddedToQueue(types::AccountIdOf<T>),

		/// Event to emit when the cancel_claim_request is ignored
		/// because the claim is already made or was never in the queue
		CancelIgnored,

		/// Wmit when a claim request have been removed from qeue
		ClaimCancelled,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_pending_claims)]
	pub type PendingClaims<T: Config> =
		StorageMap<_, Identity, types::AccountIdOf<T>, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_ice_snapshot_map)]
	pub(super) type IceSnapshotMap<T: Config> =
		StorageMap<_, Identity, T::AccountId, types::SnapshotInfo<T>, OptionQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// This error will occur when signature validation failed.
		InvalidSignature,

		/// Error to return when unauthorised operation is attempted
		DeniedOperation,

		/// Not all data required are supplied with
		IncompleteData,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Dispatchable to be called by user when they want to
		/// make a claim request
		//
		// TODO:
		// Now, we are checking the validation inside the dispatchable
		// however substrate provide a way to filter the extrinsic call
		// even before they are kept in transaction pool
		// so if we can do that, we don't have to check for signature_validation
		// here ( we will be checking that before putting the call to pool )
		// this will filter the invalid call before that are kept in pool so
		// allowing valid transaction to take over which inturn improve
		// node performance
		#[pallet::weight(10_000)]
		pub fn claim_request(
			origin: OriginFor<T>,
			icon_address: types::IconAddress,
			message: Vec<u8>,
			icon_signature: Vec<u8>,
		) -> DispatchResult {
			let ice_address = ensure_signed(origin)?;

			// make sure the validation is correct
			Self::validate_signature(
				hex::encode(ice_address.encode()).as_bytes(),
				&icon_address,
				&icon_signature,
				&message,
			)
			.map_err(|err| {
				log::info!("Signature validation failed with: {:?}", err);
				Error::<T>::InvalidSignature
			})?;

			let is_already_on_map = <IceSnapshotMap<T>>::contains_key(&ice_address);
			let is_already_on_queue = <IceSnapshotMap<T>>::contains_key(&ice_address);

			if !is_already_on_map {
				let new_snapshot = types::SnapshotInfo::<T>::default();
				<IceSnapshotMap<T>>::insert(&ice_address, new_snapshot);
				Self::deposit_event(Event::<T>::AddedToMap(ice_address.clone()));
			} else {
				Self::deposit_event(Event::<T>::SkippedAddingToMap(ice_address.clone()));
			}

			if !is_already_on_queue {
				// TODO:
				// while adding () value to map, it emits an error for Encode(Like)
				// been not implemented. Maybe the commit of this dependency do not implement for ()?
				// Not sure. Figure out the error and solve
				// <IceSnapshotMap<T>>::insert(&ice_address, ());
				Self::deposit_event(Event::<T>::AddedToQueue(ice_address));
			} else {
				Self::deposit_event(Event::<T>::SkippedAddingToQueue(ice_address));
			}

			Ok(())
		}

		/// Dispatchable that allows to cancel the claim request
		/// This function is only callable by these party:
		/// - A root origin
		/// - Owner of the account being removed
		/// - Account authorised as configured in this palellet
		#[pallet::weight(5_000)]
		pub fn cancel_claim_request(
			origin: OriginFor<T>,
			to_remove: types::AccountIdOf<T>,
		) -> DispatchResult {
			log::info!("Inside cancel_claim_request...");

			let is_authorised = {
				match ensure_root(origin.clone()) {
					// Root is always authorised
					Ok(()) => true,

					Err(_) => {
						// Non-root origin must be signed to be authorised
						match ensure_signed(origin) {
							Ok(signer) => {
								let is_sudo = signer == <SudoAccount<T>>::get();
								let is_owner = signer == to_remove;

								// Signer should be either the sudoAccount configured in this pallet
								// or should be same as the address to remove ( owner )
								is_sudo || is_owner
							}
							Err(_) => false,
						}
					}
				}
			};

			// If not authorised fail with non-authorised error
			if !is_authorised {
				fail!(Error::<T>::DeniedOperation);
			}

			let is_in_queue = <PendingClaims<T>>::contains_key(&to_remove);
			if is_in_queue {
				// If exists in queue remove it then emit respective Event
				<PendingClaims<T>>::remove(to_remove);
				Self::deposit_event(Event::<T>::ClaimCancelled);
			} else {
				// If this address do exists in queue, remove it
				// Otherwise, emit an event to inform that claim already been made
				// or was never in request queue
				Self::deposit_event(Event::<T>::CancelIgnored);
			}

			Ok(())
		}

		/// Dispatchable to transfer the fund from system balance to given address
		/// As this transfer the system balance, this must only be called within
		/// the runtime, so the origin must be root (call with Raw root or sudo_pallet)
		#[pallet::weight(0)]
		pub fn transfer_amount(
			origin: OriginFor<T>,
			receiver: types::AccountIdOf<T>,
			server_response: types::ServerResponse,
		) -> DispatchResult {
			// should only be called by root
			ensure_root(origin).map_err(|_| Error::<T>::DeniedOperation)?;

			log::info!(
				"Crediting {} amount to {:?}",
				server_response.amount,
				receiver
			);

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			// If this is not the block to start offchain worker
			// print a log and early return
			if !Self::should_run_on_this_block(&block_number) {
				log::info!("Offchain worker skipped for block: {:?}", block_number);
				return;
			}

			const CLAIMS_PER_OCW: usize = 100;

			// Get the first CLAIMS_PER_OCW number of claim request from request queue
			// As queue only contains ice_address, we also need to collect
			//
			// Destructing the process:
			// 1) get all the key-value pair of PendingClaims Storage ( in Lexicographic order )
			// 2) filter out the key (ice_address) if that ice_addess claim_status is true in snapshot map
			// 3) From the key value pair only collect key (ice_address) ignoring value (nill ())
			let claims_to_process: Vec<types::AccountIdOf<T>> = <PendingClaims<T>>::iter()
				.filter_map(|(ice_address, _nill)| {
					let snapshot = Self::get_ice_snapshot_map(&ice_address);
					match snapshot {
						Some(snapshot) if !snapshot.claim_status => Some(ice_address),
						_ => None,
					}
				})
				.take(CLAIMS_PER_OCW)
				.collect();

			// for each claims taken, call the process_claim function where actual claiming is done
			// and display weather the process been succeed
			for claimer in claims_to_process.into_iter() {
				let claim_res = Self::process_claim_request(claimer.clone());
				if let Err(err) = claim_res {
					log::info!("process_claim_request failed with error: {:?}", err);
				} else {
					log::info!("Process claim request for {:?} passed..", claimer);
				}
			}
		}
	}

	// implement all the helper function that are called from pallet hooks like offchain_worker
	impl<T: Config> Pallet<T> {
		// Function to proceed single claim request
		pub fn process_claim_request(
			ice_address: types::AccountIdOf<T>,
		) -> Result<(), types::ClaimError> {
			use types::{ClaimError, ServerError};

			// Get the icon address corresponding to this ice_address
			let icon_address = Self::get_ice_snapshot_map(&ice_address)
				.ok_or(ClaimError::NoIconAddress)?
				.icon_address;

			let server_response_res = Self::fetch_from_server(icon_address);

			// let call_to_make: CallType;
			match server_response_res {
				Err(err) => {
					match err {
						// This icon address do not exists in serevr
						// so we might just delete it from queue
						ClaimError::ServerError(err) if err == ServerError::NonExistentData => {
							// TODO:
							// set the call_to_make to point to cancel_claim_request
							// call_to_make = Self::Call::cancel_claim_request {
							// 	to_remove: ice_address.clone(),
							// };
						}

						// we might not have to handle other error espically
						err => return Err(err),
					}
				}
				Ok(response) => {
					// TODO:
					// set the call_to_make to point to transfer_function
					// call_to_make = Self::Call::transfer_amount {
					// 	to_remove: ice_address.clone(),
					// 	server_details: server_response
					// };
				}
			}

			use frame_system::offchain::{SendSignedTransaction, Signer};

			let signer = Signer::<T, T::AuthorityId>::any_account();
			let send_tx_res =
				signer.send_signed_transaction(move |_accnt| Call::cancel_claim_request {
					to_remove: ice_address.clone(),
				});

			if let Some((_account, dispatch_res)) = send_tx_res {
				if dispatch_res.is_ok() {
					Ok(())
				} else {
					Err(ClaimError::FailedExtrinsic)
				}
			} else {
				Err(ClaimError::CantDispatch)
			}
		}

		/// This function fetch the data from server and return it in required struct
		pub fn fetch_from_server(
			icon_address: types::IconAddress,
		) -> Result<types::ServerResponse, types::ClaimError> {
			use codec::alloc::string::String;
			use sp_runtime::offchain::{http, Duration};
			use types::ClaimError;

			const FETCH_TIMEOUT_PERIOD_MS: u64 = 4_0000;
			let timeout =
				sp_io::offchain::timestamp().add(Duration::from_millis(FETCH_TIMEOUT_PERIOD_MS));

			let request_url = String::from_utf8(
				T::FetchIconEndpoint::get()
					.as_bytes()
					.iter()
					// always prefix the icon_address with 0x
					.chain(b"0x")
					// we have to first bring icon_address in hex format
					.chain(hex::encode(icon_address).as_bytes())
					.cloned()
					.collect(),
			)
			// we can expect as only possibility to have error will be non-utf8 bytes in icon_address
			// which should not be possible
			.expect("Error while creating dynamic url in pallet_airdrop::fetch_from_server()");

			let request = http::Request::get(request_url.as_str());
			let pending = request
				.deadline(timeout)
				.send()
				.map_err(|_e| ClaimError::HttpError)?;
			let response = pending
				.try_wait(timeout)
				.map_err(|_e| ClaimError::HttpError)?
				.map_err(|_e| ClaimError::HttpError)?;

			// try to get the response bytes if server returned 200 code
			// or just return early
			let response_bytes: Vec<u8>;
			if response.code == 200 {
				response_bytes = response.body().collect();
			} else {
				return Err(ClaimError::HttpError);
			}

			// first try to deserialize into expected ok struct
			// then try to deserialize into known error
			// else error an invalid format
			//
			let deserialize_response_res =
				serde_json::from_slice::<types::ServerResponse>(response_bytes.as_slice());
			match deserialize_response_res {
				Ok(response) => Ok(response),
				Err(_) => {
					let deserialize_error_res =
						serde_json::from_slice::<types::ServerError>(response_bytes.as_slice());

					match deserialize_error_res {
						Ok(server_error) => Err(ClaimError::ServerError(server_error)),
						Err(_) => Err(ClaimError::InvalidResponse),
					}
				}
			}
		}

		/// Return an indicater (bool) on weather the offchain worker
		/// should be run on this block number or not
		pub fn should_run_on_this_block(block_number: &T::BlockNumber) -> bool {
			// This is the number of ocw-run block to skip after running offchain worker
			// Eg: if block is ran on block_number=3 then
			// run offchain worker in 3+ENABLE_IN_EVERY block
			const ENABLE_IN_EVERY: u32 = 3;

			*block_number % ENABLE_IN_EVERY.into() == 0_u32.into()
		}
	}

	// implement all the helper function that are called from pallet dispatchable
	impl<T: Config> Pallet<T> {
		/// Function to make sure that icon_address, ice_address and message are in sync
		/// On a high level, it does so by checking for these two verification
		/// * Make sure that icon_signature is equal to or greater than 65
		/// * Make sure that the ice_address encoded in the message and passed
		///    in this function (i.e dispatchable from where this function is called)
		///    are same
		/// * Make sure that this message is signed by the same icon_address
		///    that is being passed to this function (i.e caller for this function)
		///
		/// @return:
		/// verbose error type which point exactly where the process failed
		///
		/// @parameter:
		/// * ice_address: ss58 encoded bytes of origin of parent dispatchable
		/// * icon_address: icon_address
		/// * icon_signature: icon signature
		/// * message: raw message
		pub fn validate_signature(
			ice_address: &[u8],
			icon_address: &types::IconAddress,
			icon_signature: &[u8],
			message: &[u8],
		) -> Result<(), types::SignatureValidationError> {
			use types::SignatureValidationError;
			const COST: u64 = 1;
			const PADDING_FOR_V: [u8; 31] = [0; 31];
			/* =======================================
					Validate the icon_signature length
			*/
			ensure!(
				icon_signature.len() >= 65,
				SignatureValidationError::InvalidIconSignature
			);
			// === verified the length of icon_signature

			/* ======================================================
				Verify that the message constains the same ice_address
				as being passed to this function
			*/
			let extracted_ice_address = {
				// TODO:
				// make sure that message will always be in expected format
				const PREFIX_LEN: usize =
					b"ice_sendTransaction.data.{method.transfer.params.{wallet.".len();
				let address_len = ice_address.len();
				&message[PREFIX_LEN..PREFIX_LEN + address_len]
			};

			ensure!(
				&ice_address == &extracted_ice_address,
				SignatureValidationError::InvalidIceAddress
			);
			// ==== Verfiied that ice_address in encoded message
			// and recived in function parameterare same

			/* ================================================
				verify thet this message is being signed by same
				icon_address as passed in this function
			*/
			let (_exit_status, message_hash) = Sha3FIPS256::execute(&message, COST)
				.map_err(|_| SignatureValidationError::Sha3Execution)?;
			let formatted_icon_signature = {
				let sig_r = &icon_signature[..32];
				let sig_s = &icon_signature[32..64];
				let sig_v = &icon_signature[64..];

				// Sig final is in the format of:
				// object hash + 31 byte padding + 1 byte v + 32 byte r + 32 byte s
				message_hash
					.iter()
					.chain(&PADDING_FOR_V)
					.chain(sig_v)
					.chain(sig_r)
					.chain(sig_s)
					.cloned()
					.collect::<Vec<u8>>()
			};

			let (_exit_status, icon_pub_key) =
				ECRecoverPublicKey::execute(&formatted_icon_signature, COST)
					.map_err(|_| SignatureValidationError::ECRecoverExecution)?;

			let (_exit_status, computed_icon_address) =
				Sha3FIPS256::execute(&icon_pub_key, COST)
					.map_err(|_| SignatureValidationError::Sha3Execution)?;

			ensure!(
				&computed_icon_address[computed_icon_address.len() - 20..]
					== icon_address.as_slice(),
				SignatureValidationError::InvalidIconAddress
			);
			// ===== It is now verified that the message is signed by same icon address
			// as passed in this function

			Ok(())
		}
	}
}

// TODO:
// Do not use this in real production
/// Temporary module to provide TestAuthId
pub mod temporary {
	use sp_core::crypto::KeyTypeId;

	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"shot");
	use codec::alloc::string::String;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}
