use super::prelude::*;

#[test]
fn pool_dispatchable_from_offchain() {
	let (mut test_ext, _state) = offchain_test_ext();

	// Basic test that single call can be put on pool
	test_ext.execute_with(|| {
		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::claim_request {
				icon_address: vec![],
				message: vec![],
				icon_signature: vec![],
			}
		));
	});

	// Test that multiple call be put on pool
	test_ext.execute_with(|| {
		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::claim_request {
				icon_address: vec![],
				message: vec![],
				icon_signature: vec![],
			}
		));

		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::register_failed_claim {
				block_number: 1_u32.into(),
				ice_address: vec![]
			}
		));

		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::claim_request {
				icon_address: vec![],
				message: vec![],
				icon_signature: vec![],
			}
		));

		assert_ok!(AirdropModule::make_signed_call(
			&pallet_airdrop::pallet::Call::donate_to_creditor {
				amount: 100_u128,
				allow_death: true
			}
		));
	});
}

#[test]
fn test_ensure_root_or_sudo() {
	minimal_test_ext().execute_with(|| {
		use sp_runtime::DispatchError::BadOrigin;

		let sudo_origin = Origin::signed(AirdropModule::get_sudo_account());
		let signed_origin = Origin::signed(sp_core::sr25519::Public([12; 32]));
		let root_origin = Origin::root();
		let unsigned_origin = Origin::none();

		let sudo_call = AirdropModule::ensure_root_or_sudo(sudo_origin);
		let root_call = AirdropModule::ensure_root_or_sudo(root_origin);
		let signed_call = AirdropModule::ensure_root_or_sudo(signed_origin);
		let unsigned_call = AirdropModule::ensure_root_or_sudo(unsigned_origin);

		assert_ok!(sudo_call);
		assert_ok!(root_call);
		assert_err!(signed_call, BadOrigin);
		assert_err!(unsigned_call, BadOrigin);
	});
}
