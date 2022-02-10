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

	use frame_support::traits::Hooks;
	use types::SignatureValidationError;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		/// This error will occur when signature validation failed.
		InvalidSignature,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {}
	}

	// implement all the helper function that are called from pallet dispatchable
	impl<T: Config> Pallet<T> {}

	// implement all the helper function that are called from pallet hooks like offchain_worker
	impl<T: Config> Pallet<T> {
		/// Function to make sure that icon_address, ice_address and message are in sync
		/// On a high level, it does so by checking for these two verification
		/// 1) Make sure that the ice_address encoded in the message and passed
		///    in this function (i.e dispatchable from where this function is called)
		///    are same
		/// 2) Make sure that this message is signed by the same icon_address
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
			message: &types::SignedMessage,
		) -> Result<(), types::SignatureValidationError> {
			use types::SignatureValidationError;
			const COST: u64 = 1;
			const PADDING_FOR_V: [u8; 31] = [0; 31];

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
				SignatureValidationError::MismatchedIceAddress
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
				&computed_icon_address[computed_icon_address.len() - 20..] == icon_address,
				SignatureValidationError::MismatchedIconAddress
			);
			// ===== It is now verified that the message is signed by same icon address
			// as passed in this function

			Ok(())
		}
	}
}
