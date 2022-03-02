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
				ice_address: samples::ACCOUNT_ID[0]
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
fn ensure_root_or_sudo() {
	minimal_test_ext().execute_with(|| {
		use sp_runtime::DispatchError::BadOrigin;

		let sudo_origin = Origin::signed(AirdropModule::get_sudo_account());
		let signed_origin = Origin::signed(not_airdrop_sudo(samples::ACCOUNT_ID[2]));
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

#[test]
fn making_correct_http_request() {
	use sp_core::offchain::testing;
	let icon_address = "0xee1448f0867b90e6589289a4b9c06ac4516a75a9";

	let (mut test_ext, state) = offchain_test_ext();
	{
		let uri = String::from_utf8(
			mock::FetchIconEndpoint::get()
				.as_bytes()
				.iter()
				.chain(icon_address.as_bytes())
				.cloned()
				.collect::<Vec<u8>>(),
		)
		.unwrap();
		let response = serde_json::to_string(&samples::SERVER_DATA[0])
			.ok()
			.map(|val| val.as_bytes().to_vec());
		state.write().expect_request(testing::PendingRequest {
			method: "GET".to_string(),
			uri,
			response,
			sent: true,
			..Default::default()
		});
	}

	test_ext.execute_with(|| {
		let icon_address = sp_core::bytes::from_hex(icon_address).unwrap();
		let fetch_res = AirdropModule::fetch_from_server(icon_address);
		assert_ok!(fetch_res);
	});
}
