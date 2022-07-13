use crate as airdrop;
use airdrop::{error, info};
use airdrop::{types, utils, Pallet as AirdropModule};
use frame_support::pallet_prelude::*;
use frame_support::traits::{Currency, ExistenceRequirement};
use sp_runtime::traits::{CheckedAdd, Convert};

// Block number after which enable to do vesting
pub const VESTING_APPLICABLE_FROM: u32 = 1u32;

pub fn do_transfer<T: airdrop::Config>(snapshot: &mut types::SnapshotInfo<T>) -> DispatchResult {
	let vesting_should_end_in = <T as airdrop::Config>::AIRDROP_VARIABLES.vesting_period;
	let defi_user = snapshot.defi_user;
	let total_amount = snapshot.amount;

	let claimer = &snapshot.ice_address;
	let creditor = AirdropModule::<T>::get_creditor_account()?;

	let instant_percentage = utils::get_instant_percentage::<T>(defi_user);
	let (mut instant_amount, vesting_amount) =
			utils::get_split_amounts::<T>(total_amount, instant_percentage).map_err(|e |{
				error!("At: get_split_amount. amount: {total_amount:?}. Instant percentage: {instant_percentage}. Reason: {e:?}");
				e
			})?;

	let (transfer_schedule, remaining_amount) = utils::new_vesting_with_deadline::<
		T,
		VESTING_APPLICABLE_FROM,
	>(vesting_amount, vesting_should_end_in.into());

	// Amount to be transferred is:
	// x% of total amount
	// + remaining amount which was not perfectly divisible
	instant_amount = {
		let remaining_amount = <T::BalanceTypeConversion as Convert<
			types::VestingBalanceOf<T>,
			types::BalanceOf<T>,
		>>::convert(remaining_amount);

		instant_amount
			.checked_add(&remaining_amount)
			.ok_or(sp_runtime::ArithmeticError::Overflow)?
	};

	let creditor_origin = <T as frame_system::Config>::Origin::from(
		frame_system::RawOrigin::Signed(creditor.clone()),
	);
	let claimer_origin = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(claimer.clone());

	match transfer_schedule {
		// Apply vesting
		Some(schedule) if !snapshot.done_vesting => {
			let vest_res = pallet_vesting::Pallet::<T>::vested_transfer(
				creditor_origin,
				claimer_origin,
				schedule,
			);

			match vest_res {
				// Everything went ok. update flag
				Ok(()) => {
					let block_number = utils::get_current_block_number::<T>();
					snapshot.done_vesting = true;
					snapshot.vesting_block_number = Some(block_number);

					info!("Vesting applied for {claimer:?} at height {block_number:?}");
				}
				// log error
				Err(err) => {
					error!("Error while applying vesting. For: {claimer:?}. Reason: {err:?}");
				}
			}
		}

		// Vesting was already done as snapshot.done_vesting is true
		Some(_) => {
			info!(
				"Skipped vesting for: {claimer:?}. Reason: {reason}",
				reason = "snapshot.done_vesting was already true"
			);
		}

		// No schedule was created
		None => {
			// If vesting is not applicable once then with same total_amount
			// it will not be applicable ever. So mark it as done.
			snapshot.done_vesting = true;

			info!("No schedule was created for: {claimer:?}. All amount transferred instantly");
		}
	}

	// if not done previously
	// Transfer the amount user is expected to receiver instantly
	if !snapshot.done_instant {
		<T as airdrop::Config>::Currency::transfer(
			&creditor,
			claimer,
			instant_amount,
			ExistenceRequirement::KeepAlive,
		)
		.map_err(|err| {
			error!("Failed to instant transfer. Claimer: {claimer:?}. Reason: {err:?}");
			err
		})?;

		// Everything went ok. Update flag
		snapshot.done_instant = true;
		snapshot.initial_transfer = instant_amount;
		snapshot.instant_block_number = Some(utils::get_current_block_number::<T>());
	} else {
		info!(
			"skipped instant transfer for {claimer:?}. Reason: {reason}",
			reason = "snapshot.done_instant was set to true already"
		);
	}

	Ok(())
}
