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
//! This is only callable by sudo/root account
//!
//! * [`register_failed_claim`]
//! This dispatchable provides a mean to register that some entry was failed to process
//! and this responsibility is to be taken by node operator.
//!
//! * [`update_processed_upto_counter`]
//! To update the mark that records upto which block of claiming has been completed
//!
//! * [`remove_from_pending_claim`]
//! To remove something from pending claim as it is no longer needed. For eg: when record
//! claiming account of this icon address is 0 token

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[cfg(test)]
pub mod mock;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// All the types and alises must be defined here
pub mod types;

/// An identifier for a type of cryptographic key.
/// For this pallet, account associated with this key must be same as
/// Key stored in pallet_sudo. So that the calls made from offchain worker
/// won't get discarded because of Denied Operation
pub const KEY_TYPE_ID: sp_runtime::KeyTypeId = sp_runtime::KeyTypeId(*b"_air");

/// Gap between on when to run offchain owrker between
/// This is the number of ocw-run block to skip after running offchain worker
/// Eg: if block is ran on block_number=3 then
/// run offchain worker in 3+ENABLE_IN_EVERY block
pub const OFFCHAIN_WORKER_BLOCK_GAP: u32 = 3;

// Maximum number of time to retry a failed processing of claim entry
// There is NO point of seeting this to high value
pub const DEFAULT_RETRY_COUNT: u8 = 2;

#[frame_support::pallet]
pub mod pallet {
	use super::types;

	use frame_support::pallet_prelude::*;
	use frame_system::{ensure_signed, pallet_prelude::*};
	use sp_std::prelude::*;

	use frame_support::traits::{Currency, ExistenceRequirement, Hooks, ReservableCurrency};
	use frame_system::offchain::CreateSignedTransaction;
	use types::IconVerifiable;

	use sp_runtime::traits::Saturating;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
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
		ClaimCancelled(types::IconAddress),

		/// Emit when claim request was done successfully
		ClaimRequestSucced(
			types::BlockNumberOf<T>,
			types::AccountIdOf<T>,
			types::IconAddress,
		),

		/// Emit when an claim request was successful and fund have been transferred
		ClaimSuccess(types::IconAddress),

		/// An entry from queue was removed
		RemovedFromQueue(types::IconAddress),

		/// Same entry is processed by offchian worker for too many times
		RetryExceed(
			types::AccountIdOf<T>,
			types::IconAddress,
			types::BlockNumberOf<T>,
		),

		/// Value of OffchainAccount sotrage have been changed
		/// Return old value and new one
		OffchainAccountChanged(Option<types::AccountIdOf<T>>, types::AccountIdOf<T>),
	}

	#[pallet::storage]
	#[pallet::getter(fn get_pending_claims)]
	pub(super) type PendingClaims<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::BlockNumber,
		Twox64Concat,
		types::IconAddress,
		u8,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_processed_upto_counter)]
	pub(super) type ProcessedUpto<T: Config> = StorageValue<_, types::BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_icon_snapshot_map)]
	pub(super) type IceSnapshotMap<T: Config> =
		StorageMap<_, Twox64Concat, types::IconAddress, types::SnapshotInfo<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_offchain_account)]
	pub(super) type OffchainAccount<T: Config> =
		StorageValue<_, types::AccountIdOf<T>, OptionQuery>;

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

		/// When a same entry is being retried for too many times
		RetryExceed,
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
			let ice_address: types::AccountIdOf<T> = ensure_signed(origin)?.into();

			// We check the claim status before hand
			let is_already_on_map = <IceSnapshotMap<T>>::contains_key(&icon_address);
			ensure!(!is_already_on_map, Error::<T>::RequestAlreadyMade);

			// make sure the validation is correct
			<types::AccountIdOf<T> as Into<<T as Config>::AccountId>>::into(ice_address.clone())
				.verify_with_icon(&icon_address, &icon_signature, &message)
				.map_err(|err| {
					log::info!("Signature validation failed with: {:?}", err);
					Error::<T>::InvalidSignature
				})?;

			Self::claim_request_unchecked(ice_address, icon_address);

			Ok(())
		}

		// Means to push claim request force fully
		// This skips signature verification
		#[pallet::weight(0)]
		pub fn force_claim_request(
			origin: OriginFor<T>,
			ice_address: types::AccountIdOf<T>,
			icon_address: types::IconAddress,
		) -> DispatchResult {
			ensure_root(origin).map_err(|_| Error::<T>::DeniedOperation)?;

			Self::claim_request_unchecked(ice_address, icon_address);

			Ok(())
		}

		/// Dispatchable to transfer the fund from system balance to given address
		/// As this transfer the system balance, this must only be called within
		/// sudo or with root origin
		// TODO:
		// Sudo pallet can have only one key at a time. This implies that
		// when sudo key is changed we also have to insert the new sudo key in the
		// keystore and rotate it so that offchain worker always use new sudo account.
		// failing to do so will bring offchain worker to a state where it can't send
		// authorised signed transaction anymore leaving all call to this function to
		// fail
		// Some solution:
		// 1) Node operator should ensure to sync sudo key and keystore as above
		// 2) Maintain list of accounts local to this pallet only, (was already done)
		//	  This seems to be good to go approach but brings about the hazard &
		//	  confusion to maintain two list of authorised accounts
		//
		// If any of the step fails in this function,
		// we pass the flow to register_failed_claim if needed to be retry again
		// and cancel the request if it dont have to retried again
		#[pallet::weight(0)]
		pub fn complete_transfer(
			origin: OriginFor<T>,
			block_number: types::BlockNumberOf<T>,
			receiver_icon: types::IconAddress,
			server_response: types::ServerResponse,
		) -> DispatchResult {
			// Make sure this is either sudo or root
			Self::ensure_root_or_offchain(origin.clone())
				.map_err(|_| Error::<T>::DeniedOperation)?;

			// Check again if it is still in the pending queue
			// Eg: If another node had processed the same request
			// or if user had decided to cancel_claim_request
			// this entry won't be present in the queue
			let is_in_queue = <PendingClaims<T>>::contains_key(&block_number, &receiver_icon);
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
				Self::get_icon_snapshot_map(&receiver_icon).ok_or(Error::<T>::IncompleteData)?;

			// Also make sure that claim_status of this snapshot is false
			ensure!(!snapshot.claim_status, Error::<T>::ClaimAlreadyMade);

			// Convert server_response.amount type ( usually u128 ) to balance type
			// this will check for underflow overflow if u128 cannot be assigned to balance
			// Failing due to this reasong implies that we are using incompatible data type
			// in ServerResponse.amount & pallet_airdrop::Currency
			let amount: types::BalanceOf<T> = server_response.amount.try_into().map_err(|_| {
				// We are moving it to keep for retry
				// But it will also fail in next time because it is eror
				// with incompatable format between server response & rust struct
				Self::register_failed_claim(origin.clone(), block_number, receiver_icon.clone())
					.expect("Calling register_failed_claim from amount.try_into.This call should not have failed.");

				DispatchError::Other("Cannot convert server_response.value into Balance type")
			})?;

			// Transfer the amount to this reciver keeping creditor alive
			T::Currency::transfer(
				&Self::get_creditor_account(),
				&snapshot.ice_address,
				amount,
				ExistenceRequirement::KeepAlive,
			)
			.map_err(|err| {
				// This is also error from our side. We keep it for next retry
				Self::register_failed_claim(origin.clone(), block_number, receiver_icon.clone()).expect("Calling register failed_claim from currency::transfer. This call should not have failed..");

				log::info!("Currency transfer failed with error: {:?}", err);
				err
			})?;

			// Update claim_status to true and store it
			snapshot.claim_status = true;
			<IceSnapshotMap<T>>::insert(&receiver_icon, snapshot);

			// Now we can remove this claim from queue
			<PendingClaims<T>>::remove(&block_number, &receiver_icon);

			Self::deposit_event(Event::<T>::ClaimSuccess(receiver_icon));

			Ok(())
		}

		/// Call to set OffchainWorker Account ( Restricted to root only )
		/// Why?
		/// We have to have a way that this signed call is from offchain so we can perform
		/// critical operation. When offchain worker key and this storage have same account
		/// then we have a way to ensure this call is from offchain worker
		#[pallet::weight(0)]
		pub fn set_offchain_account(
			origin: OriginFor<T>,
			new_account: types::AccountIdOf<T>,
		) -> DispatchResult {
			ensure_root(origin).map_err(|_| Error::<T>::DeniedOperation)?;

			let old_value = Self::get_offchain_account();
			<OffchainAccount<T>>::set(Some(new_account.clone()));
			Self::deposit_event(Event::OffchainAccountChanged(old_value, new_account));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn remove_from_pending_queue(
			origin: OriginFor<T>,
			block_number: types::BlockNumberOf<T>,
			icon_address: types::IconAddress,
		) -> DispatchResult {
			Self::ensure_root_or_offchain(origin)?;

			<PendingClaims<T>>::remove(&block_number, &icon_address);
			Self::deposit_event(Event::<T>::RemovedFromQueue(icon_address));

			Ok(())
		}

		/// Call that handles what to do when an entry failed while
		/// processing in offchain worker
		/// We move the entry to future block key so that another
		/// offchain worker can process it again
		#[pallet::weight(0)]
		pub fn register_failed_claim(
			origin: OriginFor<T>,
			block_number: types::BlockNumberOf<T>,
			icon_address: types::IconAddress,
		) -> DispatchResult {
			Self::ensure_root_or_offchain(origin).map_err(|_| Error::<T>::DeniedOperation)?;

			let ice_address = Self::get_icon_snapshot_map(&icon_address)
				.ok_or(Error::<T>::IncompleteData)?
				.ice_address;
			let retry_remaining = Self::get_pending_claims(&block_number, &icon_address)
				.ok_or(Error::<T>::NotInQueue)?;

			// In both case weather retry is remaining or not
			// we will remove this entry from this block number key
			// so do it early to make sure we dont miss this inany case
			// otherwise this entry will always be floating around in storage
			<PendingClaims<T>>::remove(&block_number, &icon_address);

			// Check if it's retry counter have been brought to zero
			// if so do not move this entry to next block
			ensure!(retry_remaining > 0, {
				log::info!(
					"Retry limit exceed for address: {:?} is in block number: {:?}",
					ice_address,
					block_number
				);

				// TODO:
				// What to do when retry exceed?
				// it is already removed from queue in previous statements
				// so is there nothing else to do?

				// Emit event and return with error
				Self::deposit_event(Event::<T>::RetryExceed(
					ice_address,
					icon_address,
					block_number,
				));
				Error::<T>::RetryExceed
			});

			// This entry have some retry remaining so we put this entry in another block_number key
			let new_block_number = Self::get_current_block_number().saturating_add(1_u32.into());
			<PendingClaims<T>>::insert(
				&new_block_number,
				&icon_address,
				retry_remaining.saturating_sub(1),
			);

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

		#[pallet::weight(0)]
		pub fn update_processed_upto_counter(
			origin: OriginFor<T>,
			new_value: types::BlockNumberOf<T>,
		) -> DispatchResult {
			Self::ensure_root_or_offchain(origin).map(|_| Error::<T>::DeniedOperation)?;

			<ProcessedUpto<T>>::set(new_value);

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: types::BlockNumberOf<T>) {
			// If this is not the block to start offchain worker
			// print a log and early return
			if !Self::should_run_on_this_block(block_number) {
				log::info!("Offchain worker skipped for block: {:?}", block_number);
				return;
			}

			// Start processing from one + the block previously processed
			let start_processing_from =
				Self::get_processed_upto_counter().saturating_add(1_u32.into());

			// Mark the block numbers we will be processing
			let mut blocks_to_process =
				types::PendingClaimsOf::<T>::new(start_processing_from..block_number);

			while let Some((block_number, mut entries_in_block)) = blocks_to_process.next() {
				while let Some(claimer) = entries_in_block.next() {
					let claim_res = Self::process_claim_request((block_number, claimer.clone()));

					if let Err(err) = claim_res {
						log::info!(
							"Claim process failed for Pending entry {:?} with error {:?}",
							(claimer, block_number),
							err
						);
					} else {
						log::info!(
							"complete_transfer function called sucessfully on Pending entry {:?}",
							(claimer, block_number)
						);
					}
				}
			}

			let update_res = Self::make_signed_call(&Call::update_processed_upto_counter {
				new_value: block_number.saturating_sub(1_u32.into()),
			});

			// We assume that calling extrinsic will always pass. If not here is worst case scanerio
			// update counter is never updated i.e always remains 0
			// and we will always try processing claims made from request
			// 0..10(no of block to process in one ocw)
			// So:
			// something should keep checking for process_upto value and notify when
			// the gap between this value and current block is too high
			// ( which signifies that this counter is not being updated lately )
			//
			// Just ignoring the result
			update_res.ok();
		}
	}

	// implement all the helper function that are called from pallet hooks like offchain_worker
	impl<T: Config> Pallet<T> {
		// Function to proceed single claim request
		pub fn process_claim_request(
			(stored_block_num, icon_address): (types::BlockNumberOf<T>, types::IconAddress),
		) -> Result<(), types::ClaimError> {
			use types::{ClaimError, ServerError};

			let server_response_res = Self::fetch_from_server(&icon_address);

			let call_to_make: Call<T>;
			match server_response_res {
				// If error is NonExistentData then, it signifies this icon address do not exists in server
				// so we just cancel the request
				Err(ClaimError::ServerError(ServerError::NonExistentData)) => {
					// TODO: should we call register_fail or remove_fom_queue?
					call_to_make = Call::register_failed_claim {
						icon_address: icon_address.clone(),
						block_number: stored_block_num,
					};
				}

				// If transferring amount is 0, then we can just cancel this claim too
				// as transferring 0 amount have no effect
				Ok(response) if response.amount == 0 => {
					// TODO: should we call register_fail or remove_from_queue
					call_to_make = Call::remove_from_pending_queue {
						icon_address: icon_address.clone(),
						block_number: stored_block_num,
					};
				}

				// If is any other error, just propagate it to caller
				Err(err) => return Err(err),

				// if response is valid, then call complete_transfer dispatchable
				// This will also clear the queue
				Ok(response) => {
					call_to_make = Call::complete_transfer {
						receiver_icon: icon_address.clone(),
						server_response: response,
						block_number: stored_block_num,
					};
				}
			}

			let call_res = Self::make_signed_call(&call_to_make);
			call_res.map_err(|err| {
				// NOTE:
				// VERY_IMPORTANT:
				// We have to make very sure that this call will always success.
				// For eg:
				// if we are about to register failed request ( which will reove the entry from current key )
				// and if this calling failed, that that entry will never be removed
				// Case 2:
				// If we are about to transfer the amount but if this function never succeed
				// then the fund will not have been transferred and user neither can do claim again
				// ( as db shows user had already done the claim )
				// And this same entry will never be processed by another offchain worker so the airdropping
				// is lost ( until we query it and do it manually )
				//
				// So we have to make sure that this transaction always reach to the transaction pool
				//
				// Possible sources of error:
				// --- 1) The account configured in local storage do not have permission to call authorised function
				// This usually happen when we update sudo key from pallet_sudo & forget to update & rotate
				// keys in keystore of this pallet, then the filted from respective call
				// `ensure_root_or_sudo()` will always fail thus performing no process
				//
				// -- 2) There is no account configured in keystore
				// This can be tested before deploying node
				// --3) Implementation itself to call signed transaction is buggy
				// -- This also have to be made sure by the implementors

				// TODO:
				// Maintain local sudo account

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
			use frame_system::offchain::SigningTypes;
			use types::CallDispatchableError;

			// If we can send this from account then only extrinsic will pass ensure_ statement
			let send_from = <<T as SigningTypes>::Public as Decode>::decode(
				&mut Self::get_offchain_account()
					.ok_or(CallDispatchableError::NoAccount)?
					.encode()
					.as_ref(),
			)
			.map_err(|err| {
				log::info!(
					"[make_signed_call] Converting to Public from AccountId failed with error: {:?}",
					err
				);
				CallDispatchableError::NoAccount
			})?;

			let signer = frame_system::offchain::Signer::<T, T::AuthorityId>::all_accounts()
				.with_filter(vec![send_from]);
			if !signer.can_sign() {
				return Err(CallDispatchableError::NoAccount);
			}

			let send_tx_res = signer.send_signed_transaction(move |_accnt| (*call_to_make).clone());

			if let Some((_accnt, send_tx_res)) = send_tx_res.iter().next() {
				send_tx_res.map_err(|_| CallDispatchableError::CantDispatch)
			} else {
				log::info!(
					"[make_signed_call] calling send_signed_transaction returned empty result."
				);
				Err(CallDispatchableError::CantDispatch)
			}
		}

		/// This function fetch the data from server and return it in required struct
		pub fn fetch_from_server(
			icon_address: &types::IconAddress,
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
		pub fn should_run_on_this_block(block_number: types::BlockNumberOf<T>) -> bool {
			block_number % crate::OFFCHAIN_WORKER_BLOCK_GAP.into() == 0_u32.into()
		}
	}

	// implement all the helper function that are called from pallet dispatchable
	impl<T: Config> Pallet<T> {
		pub fn get_creditor_account() -> types::AccountIdOf<T> {
			use sp_runtime::traits::AccountIdConversion;

			T::Creditor::get().into_account()
		}

		/// Do claim request withing checking for anything.
		/// This is seperated as a means to share logic
		/// And always should be called only after doing proper check before hand
		pub fn claim_request_unchecked(
			ice_address: types::AccountIdOf<T>,
			icon_address: types::IconAddress,
		) {
			// Get the current block number. This is the number where user asked for claim
			// and we store it in PencingClaims to preserve FIFO
			let current_block_number = Self::get_current_block_number();

			// Insert with default snapshot but with real icon address mapping
			let new_snapshot = types::SnapshotInfo::<T>::default().ice_address(ice_address.clone());
			<IceSnapshotMap<T>>::insert(&icon_address, new_snapshot);

			// insert in queue respective to current block number
			// and retry as defined in crate level constant
			<PendingClaims<T>>::insert(
				&current_block_number,
				&icon_address,
				crate::DEFAULT_RETRY_COUNT,
			);

			Self::deposit_event(Event::<T>::ClaimRequestSucced(
				current_block_number,
				ice_address,
				icon_address,
			));
		}

		/// Helper function to create similar interface like `ensure_root`
		/// but which instead check for sudo key
		pub fn ensure_root_or_offchain(origin: OriginFor<T>) -> DispatchResult {
			let is_root = ensure_root(origin.clone()).is_ok();
			let is_offchain = {
				let signed = ensure_signed(origin);
				signed.is_ok() && signed.ok() == Self::get_offchain_account()
			};

			ensure!(is_root || is_offchain, DispatchError::BadOrigin);
			Ok(())
		}

		/// Return block height of Node from which this was called
		pub fn get_current_block_number() -> types::BlockNumberOf<T> {
			<frame_system::Pallet<T>>::block_number()
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
				.map_err(|_| SignatureValidationError::InvalidIconSignature)?;

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
