use crate as airdrop;
use airdrop::{error, info};
use airdrop::{types, utils, Pallet as AirdropModule};
use frame_support::pallet_prelude::*;
use frame_support::traits::{Currency, ExistenceRequirement};

pub struct AllInstantTransfer;

impl types::DoTransfer for AllInstantTransfer {
	fn do_transfer<T: airdrop::Config>(snapshot: &mut types::SnapshotInfo<T>) -> DispatchResult {
		let creditor = AirdropModule::<T>::get_creditor_account()?;
		let claimer = &snapshot.ice_address;
		let total_balance = snapshot.amount;

		if !snapshot.done_instant {
			<T as airdrop::Config>::Currency::transfer(
				&creditor,
				&claimer,
				total_balance,
				ExistenceRequirement::KeepAlive,
			)
			.map_err(|e| {
				info!("At: AllInstant::do_transfer. Claimer: {claimer:?}. Reason: {e:?}");
				e
			})?;

			// Everything went ok. Update flag
			snapshot.done_instant = true;
			snapshot.initial_transfer = total_balance;
			snapshot.instant_block_number = Some(utils::get_current_block_number::<T>());
		} else {
			info!(
				"At: AllInstantTransfer::do_transfer. Skipped for claimer: {claimer:?}.{reason}",
				reason = "snapshot.done_instant was true already"
			);
		}

		Ok(())
	}
}
