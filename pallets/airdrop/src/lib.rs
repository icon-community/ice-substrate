//! # Airdrop pallet
//! This pallet is responsible to do airdropping to claimers of icon_address.
//! - [`Config`]
//! - [`Call`]
//!
//! ## Interface
//!
//! ### Disptachable function for end-user
//!
//! * [`claim_request`]
//!
//! Signed call that accepts set of ice data (icon signature, initial message, icon address).
//! This maps the ice address with icon address ( if previously not done)
//! and register this request in pending claims.
//!
//! * [`cancel_request`]
//!
//! Allows any user to cancel last claim request. This does so by just removing
//! respective entry from pending queue. Only callable by the owner who
//! first called `claim_request` or root account or palelt authorised account.
//!
//! * [`donateto_creditor`]
//!
//! Allows any user to donate funds to the pallet account. Pallet account is
//! the account from which the airdrop funds will be transferred from.
//! This dispatchable should be called to credit pallet with enough balance
//! else every call to `complete_transfer` will fail. This should be done before any
//! user arrives to node for claiming request
//!
//! ---
//! ### Dispatchable calls for internal use
//! * [`complete_transfer`]
//!
//! This transfer the fund to given ice address with given transaction details
//! inside `ServerResponse` type. If transferring the fund succeed, it will also
//! remove the queue from pendingClaims and update any snapshot info as needed.
//!
//! This is only callable by pallet internal Authorised account
//!
//! ## Genesis Config
//! SudoAccount of this pallet ( not `pallet_sudo` ) needs to be configured in genesis config
//! This can intentionally be made into sudo key of pallet_sudo. But not that changing of
//! `pallet_sudo`'s key ( i.e calling `pallet_sudo::Call::set_key` ) will leave this unaffected
//! A similar interafce [`set_authorised`] is provided to set new account to be authorised.
//! Note: Only one account is authorised at a time

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

/// An identifier for a type of cryptographic key.
/// For this pallet, account associated with this key must be same as
/// Key stored in pallet_sudo. So that the calls made from offchain worker
/// won't get discarded because of Denied Operation
pub const KEY_TYPE_ID: sp_runtime::KeyTypeId = sp_runtime::KeyTypeId(*b"_air");

#[frame_support::pallet]
pub mod pallet {
	use super::types;

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	use frame_support::traits::{Currency, ExistenceRequirement, Hooks, ReservableCurrency};
	use frame_system::offchain::CreateSignedTransaction;
	use types::IconVerifiable;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + CreateSignedTransaction<Call<Self>> + pallet_sudo::Config
	{
		/// AccountIf type that is same as frame_system's accountId also
		/// extended to be verifable against icon data
		type AccountId: IconVerifiable + IsType<<Self as frame_system::Config>::AccountId>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<types::AccountIdOf<Self>>
			+ ReservableCurrency<types::AccountIdOf<Self>>;

		/// The overarching dispatch call type.
		// type Call: From<Call<Self>>;

		/// The identifier type for an offchain worker.
		type AuthorityId: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>;

		/// Endpoint on where to send request url
		#[pallet::constant]
		type FetchIconEndpoint: Get<&'static str>;

		/// Id of account from which to send fund to claimers
		/// This account should be credited enough to supply fund for all claim requests
		#[pallet::constant]
		type Creditor: Get<frame_support::PalletId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emit when a claim request have been removed from queue
		ClaimCancelled(types::AccountIdOf<T>),

		/// Emit when claim request was done successfully
		ClaimRequestSucced(types::AccountIdOf<T>),

		/// Emit when an claim request was successful and fund have been transferred
		ClaimSuccess(types::AccountIdOf<T>),
	}

	#[pallet::storage]
	#[pallet::getter(fn get_pending_claims)]
	pub(super) type PendingClaims<T: Config> =
		StorageMap<_, Identity, types::AccountIdOf<T>, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_ice_snapshot_map)]
	pub(super) type IceSnapshotMap<T: Config> =
		StorageMap<_, Identity, types::AccountIdOf<T>, types::SnapshotInfo<T>, OptionQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// This error will occur when signature validation failed.
		InvalidSignature,

		/// Error to return when unauthorised operation is attempted
		DeniedOperation,

		/// Not all data required are supplied with
		IncompleteData,

		/// When expected data is not present in queue
		NotInQueue,

		/// Claim has already been made so can't be made again at this time
		ClaimAlreadyMade,

		/// This request have been already made.
		RequestAlreadyMade,
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
			// Take signed address compatible with airdrop_pallet::Config::AccountId type
			// so that we can call verify_with_icon method
			let ice_address: <T as Config>::AccountId = ensure_signed(origin)?.into();

			// make sure the validation is correct
			ice_address
				.verify_with_icon(&icon_address, &icon_signature, &message)
				.map_err(|err| {
					log::info!("Signature validation failed with: {:?}", err);
					Error::<T>::InvalidSignature
				})?;

			// Convert back to to frame_system::Config::AccountId
			let ice_address: types::AccountIdOf<T> = ice_address.into();

			/*
			TODO:
			We might have to check both is_in_queue & is_in_map  independently
			and do not error on absence of either of them.
			Consider a activity when:
			- Use make claim_request ( which will add to map & queue )
			- Cancel the claim request ( which will remove from queue & data in map is preserved anyway )
			- then user will never be able to claim again ( because data in already on map & we are throwing error on this condition )
			*/
			let is_already_on_map = <IceSnapshotMap<T>>::contains_key(&ice_address);
			ensure!(!is_already_on_map, Error::<T>::RequestAlreadyMade);

			let new_snapshot = types::SnapshotInfo::<T>::default().icon_address(icon_address);
			<IceSnapshotMap<T>>::insert(&ice_address, new_snapshot);
			<PendingClaims<T>>::insert(&ice_address, ());

			Self::deposit_event(Event::<T>::ClaimRequestSucced(ice_address));
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
			// If this origin is either root, or the signed by sudo key then this is authorised
			let is_sudo_or_root = Self::ensure_root_or_sudo(origin.clone()).is_ok();
			let is_owner = ensure_signed(origin).as_ref().ok() == Some(&to_remove);

			ensure!(is_sudo_or_root || is_owner, Error::<T>::DeniedOperation);

			let is_in_queue = <PendingClaims<T>>::contains_key(&to_remove);
			ensure!(is_in_queue, Error::<T>::NotInQueue);

			<PendingClaims<T>>::remove(&to_remove);
			Self::deposit_event(Event::<T>::ClaimCancelled(to_remove));

			Ok(())
		}

		/// Dispatchable to transfer the fund from system balance to given address
		/// As this transfer the system balance, this must only be called within
		/// sudo or with root origin
		#[pallet::weight(0)]
		pub fn complete_transfer(
			origin: OriginFor<T>,
			receiver: types::AccountIdOf<T>,
			server_response: types::ServerResponse,
		) -> DispatchResult {
			// Make sure this is either sudo or root
			Self::ensure_root_or_sudo(origin).map_err(|_| Error::<T>::DeniedOperation)?;

			// Check again if it is still in the pending queue
			// Eg: If another node had processed the same request
			// or if user had decided to cancel_claim_request
			// this entry won't be present in the queue
			let is_in_queue = <PendingClaims<T>>::contains_key(&receiver);
			ensure!(is_in_queue, {
				log::info!(
					"{}{}",
					"The claim was no longer in queue",
					"May be user clanceled or another node did it already?"
				);
				Error::<T>::NotInQueue
			});

			// Get snapshot from map and return with error if not present
			let mut snapshot =
				Self::get_ice_snapshot_map(&receiver).ok_or(Error::<T>::IncompleteData)?;

			// Also make sure that claim_status of this snapshot is false
			ensure!(!snapshot.claim_status, Error::<T>::ClaimAlreadyMade);

			// Convert server_response.amount type ( usually u128 ) to balance type
			// this will check for underflow overflow if u128 cannot be assigned to balance
			// Failing due to this reasong implies that we are using incompatible data type
			// in ServerResponse.amount & pallet_airdrop::Currency
			let amount = server_response.amount.try_into().map_err(|_| {
				log::info!("Invalid amount value....");
				DispatchError::Other("Cannot convert server_response.value into Balance type")
			})?;

			// Transfer the amount to this reciver keeping creditor alive
			T::Currency::transfer(
				&Self::get_creditor_account(),
				&receiver,
				amount,
				ExistenceRequirement::KeepAlive,
			)
			.map_err(|err| {
				log::info!("Currency transfer failed with error: {:?}", err);
				err
			})?;

			// Update claim_status to true and store it
			snapshot.claim_status = true;
			<IceSnapshotMap<T>>::insert(&receiver, snapshot);

			// Now we can remove this claim from queue
			<PendingClaims<T>>::remove(&receiver);

			log::info!(
				"Complete_tranfser function on ice address: {:?} passed..",
				receiver
			);

			Self::deposit_event(Event::<T>::ClaimSuccess(receiver));

			Ok(())
		}

		/// Public function to deposit some fund for our creditor
		/// @parameter:
		/// - origin: Signed Origin from which to credit
		/// - amount: Amount to donate
		/// - allow_death: when transferring amount,
		/// 		if origin's balance drop below minimum balance
		/// 		then weather to transfer (resulting origin account to vanish)
		/// 		or cancel the donation
		/// This function can be used as a mean to credit our creditor if being donated from
		/// any node operator owned account
		#[pallet::weight(0)]
		pub fn donate_to_creditor(
			origin: OriginFor<T>,
			amount: types::BalanceOf<T>,
			allow_death: bool,
		) -> DispatchResult {
			let sponser = ensure_signed(origin)?;

			let creditor_account = Self::get_creditor_account();
			let existance_req = if allow_death {
				ExistenceRequirement::AllowDeath
			} else {
				ExistenceRequirement::KeepAlive
			};

			T::Currency::transfer(&sponser, &creditor_account, amount, existance_req)?;

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
			// TODO:: Figure out way to process claims in first come basis
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

			let call_to_make: Call<T>;
			match server_response_res {
				// If error is NonExistentData then, it signifies this icon address do not exists in server
				// so we just cancel the request
				Err(ClaimError::ServerError(ServerError::NonExistentData)) => {
					call_to_make = Call::cancel_claim_request {
						to_remove: ice_address.clone(),
					};
				}

				// If transferring amount is 0, then we can just cancel this claim too
				// as transferring 0 amount have no effect
				Ok(response) if response.amount == 0 => {
					call_to_make = Call::cancel_claim_request {
						to_remove: ice_address.clone(),
					};
				}

				// If is any other error, just propagate it to caller
				Err(err) => return Err(err),

				// if response is valid, then call complete_transfer dispatchable
				// This will also clear the queue
				Ok(response) => {
					call_to_make = Call::complete_transfer {
						receiver: ice_address.clone(),
						server_response: response,
					};
				}
			}

			let call_res = Self::make_signed_call(&call_to_make);
			call_res.map_err(|err| {
				log::info!(
					"Calling extrinsic {:#?} failed with error: {:?}",
					call_to_make,
					err
				);
				ClaimError::CallingError(err)
			})
		}

		/// Helper function to send signed transaction to provided callback
		/// and map the resulting error
		/// @return:
		/// - Error or the account from which transaction was made
		/// NOTE:
		/// As all call are sent from here, it is important to verify that ::any_account()
		/// in this context returns accounts that are authorised
		/// i.e present in Storage::SudoAccount
		/// So that the call is not discarded inside the calling function if it checks for so
		/// This can be done by intentionally if done:
		/// rpc call to: author.insertKey(crate::KEY_TYPE_ID, _, ACCOUNT_X)
		/// extrensic call to: Self::set_authorised(ACCOUNT_X) // Same as above account
		pub fn make_signed_call(
			call_to_make: &Call<T>,
		) -> Result<(), types::CallDispatchableError> {
			use frame_system::offchain::SendSignedTransaction;
			use types::CallDispatchableError;

			let signer = frame_system::offchain::Signer::<T, T::AuthorityId>::any_account();
			let send_tx_res = signer.send_signed_transaction(move |_accnt| (*call_to_make).clone());

			if let Some((_account, dispatch_res)) = send_tx_res {
				if dispatch_res.is_ok() {
					Ok(())
				} else {
					Err(CallDispatchableError::CantDispatch)
				}
			} else {
				Err(CallDispatchableError::NoAccount)
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

			log::info!("Sending request to: {}", request_url);
			let request = http::Request::get(request_url.as_str());

			log::info!("Initilizing pending variable..");
			let pending = request.deadline(timeout).send().map_err(|e| {
				log::info!("While pending error: {:?}", e);
				ClaimError::HttpError
			})?;

			log::info!("Initilizing response variable..");
			let response = pending
				.try_wait(timeout)
				.map_err(|e| {
					log::info!("First error: {:?}...", e);
					ClaimError::HttpError
				})?
				.map_err(|e| {
					log::info!("Second error: {:?}", e);
					ClaimError::HttpError
				})?;

			// try to get the response bytes if server returned 200 code
			// or just return early
			let response_bytes: Vec<u8>;
			if response.code == 200 {
				response_bytes = response.body().collect();
			} else {
				log::info!("Unexpected http code: {}", response.code);
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
		pub fn get_creditor_account() -> types::AccountIdOf<T> {
			use sp_runtime::traits::AccountIdConversion;

			T::Creditor::get().into_account()
		}

		/// return the key set in sudo pallet
		#[inline(always)]
		pub fn get_sudo_account() -> types::AccountIdOf<T> {
			pallet_sudo::Pallet::<T>::key()
		}

		/// Helper function to create similar interface like `ensure_root`
		/// but which instead check for sudo key
		pub fn ensure_root_or_sudo(origin: OriginFor<T>) -> DispatchResult {
			let is_root = ensure_root(origin.clone()).is_ok();
			let is_sudo = ensure_signed(origin).ok() == Some(Self::get_sudo_account());

			ensure!(is_root || is_sudo, DispatchError::BadOrigin);
			Ok(())
		}
	}
}

pub mod airdrop_crypto {
	use crate::KEY_TYPE_ID;

	use codec::alloc::string::String;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE_ID);

	pub struct AuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

/// Implement IconVerifiable for Anything that can be decoded into Vec<u8>
// However not that
impl types::IconVerifiable for sp_runtime::AccountId32 {
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
	fn verify_with_icon(
		&self,
		icon_wallet: &types::IconAddress,
		icon_signature: &[u8],
		message: &[u8],
	) -> Result<(), types::SignatureValidationError> {
		use codec::Encode;
		use fp_evm::LinearCostPrecompile;
		use frame_support::ensure;
		use pallet_evm_precompile_sha3fips::Sha3FIPS256;
		use pallet_evm_precompile_simple::ECRecoverPublicKey;
		use types::SignatureValidationError;

		const COST: u64 = 1;
		const PADDING_FOR_V: [u8; 31] = [0; 31];

		let ice_address = hex::encode(self.encode());
		let ice_address = ice_address.as_bytes();

		/* =======================================
				Validate the icon_signature length
		*/
		ensure!(
			icon_signature.len() == 65,
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
				.collect::<sp_std::vec::Vec<u8>>()
		};

		let (_exit_status, icon_pub_key) =
			ECRecoverPublicKey::execute(&formatted_icon_signature, COST)
				.map_err(|_| SignatureValidationError::ECRecoverExecution)?;

		let (_exit_status, computed_icon_address) = Sha3FIPS256::execute(&icon_pub_key, COST)
			.map_err(|_| SignatureValidationError::Sha3Execution)?;

		ensure!(
			&computed_icon_address[computed_icon_address.len() - 20..] == icon_wallet.as_slice(),
			SignatureValidationError::InvalidIconAddress
		);
		// ===== It is now verified that the message is signed by same icon address
		// as passed in this function

		Ok(())
	}
}
