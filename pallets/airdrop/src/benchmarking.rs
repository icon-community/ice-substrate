//! Benchmarking setup for pallet-airdrop

use super::*;

use crate as pallet_airdrop;
#[allow(unused)]
use crate::Pallet;
use codec::Decode;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{pallet_prelude::*, traits::ConstU32, BoundedVec};
use frame_system::RawOrigin;
use sp_core::sr25519;
use sp_core::*;
use sp_runtime::traits::Convert;
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::traits::Saturating;
use sp_std::prelude::*;
use types::{AccountIdOf, BlockNumberOf, IceAddress, MerkleHash, RawPayload};

#[derive(Clone, Debug)]
pub struct BenchmarkSample<'a> {
	pub icon_address: &'a str,
	pub ice_address: &'a str,
	pub message: &'a str,
	pub icon_signature: &'a str,
	pub ice_signature: &'a str,
	pub amount: u128,
	pub defi_user: bool,
	pub merkle_proofs: &'a [&'a str],
	pub merkle_root: &'a str,
}

#[derive(Clone, Debug)]
pub struct UserClaimTestCase<T: Get<u32>> {
	pub icon_address: types::IconAddress,
	pub ice_address: IceAddress,
	pub message: RawPayload,
	pub icon_signature: types::IconSignature,
	pub ice_signature: types::IceSignature,
	pub amount: u128,
	pub defi_user: bool,
	pub merkle_proofs: BoundedVec<MerkleHash, T>,
	pub merkle_root: [u8; 32],
}

impl<'a, B> From<BenchmarkSample<'a>> for UserClaimTestCase<B>
where
	B: Get<u32>,
{
	fn from(sample: BenchmarkSample<'a>) -> Self {
		let proff_size = B::get();

		// assert_eq!(sample.merkle_proofs.len(), proff_size as usize);
		let amount = sample.amount;
		let defi_user = sample.defi_user;
		let ice_address = hex::decode(sample.ice_address).unwrap().try_into().unwrap();
		let merkle_root = hex::decode(sample.merkle_root).unwrap().try_into().unwrap();
		let message = sample.message.as_bytes().to_vec().try_into().unwrap();
		let icon_address = hex::decode(sample.icon_address)
			.unwrap()
			.try_into()
			.unwrap();
		let icon_signature = hex::decode(sample.icon_signature)
			.unwrap()
			.try_into()
			.unwrap();
		let ice_signature = hex::decode(sample.ice_signature)
			.unwrap()
			.try_into()
			.unwrap();
		let merkle_proofs = sample
			.merkle_proofs
			.iter()
			.map(|proff| {
				let proff = hex::decode(proff).unwrap();
				proff.try_into().unwrap()
			})
			.collect::<Vec<MerkleHash>>()
			.try_into()
			.unwrap();

		UserClaimTestCase::<B> {
			icon_address,
			ice_address,
			message,
			merkle_root,
			icon_signature,
			ice_signature,
			amount,
			defi_user,
			merkle_proofs,
		}
	}
}

pub const benchmark_samples: [BenchmarkSample; 4] = [
	   BenchmarkSample{
		icon_address : "3d16047c23cc3e27e807f6cfc55fb8d950555690",
		icon_signature : "a9b435d7720228efd12327a3551ffab56e737be8071673cfdad59f25a9d20f425b99874ac4c17b29063160d247c95cc0a71b3647e58b5dfb67c1839943920b9700",
		message : "icx_sendTransaction.data.{method.transfer.params.{wallet.9e1d877d67c30235bc90e0ff469d0f8c2b4d7df3d8efed7873715cd9da073f2f}}.dataType.call.from.hx3d16047c23cc3e27e807f6cfc55fb8d950555690.nid.0x1.nonce.0x1.stepLimit.0x0.timestamp.0x0.to.hx3d16047c23cc3e27e807f6cfc55fb8d950555690.version.0x3",
		ice_address : "9e1d877d67c30235bc90e0ff469d0f8c2b4d7df3d8efed7873715cd9da073f2f",
		ice_signature : "2425488fc058d9e2810ebda2e6a1466ac1f159f0db8b807037132759257131602378cba5d65859be2e3a44e5d6da91bad14dd0ead745ea159551fa748e8f1285",
		amount : 1000000000000000000,
		defi_user : true,
		merkle_proofs : &[
			"e0d3b76446424001e3ed793dce6e3768d23330c44108dc1af0b6618c291bd9ea",
			"31a8941be7278d42e60bb911e7d1ec34c20e22c97e5de8d4523b0b156552de32",
			"09eaef8a1f8fb9a3709a155eb40abb05164b33c20089a99ac9018e8fb372e708",
			"b7e0d8418a2aba80ce033ca7fa6ceb446b2b3a0ab4f13b89559a24938f7006e1",
			"8313ec772d670751b1d5325513953347874381c068edfb2fce6aca1556754be5",
			"1ab85582900d947bb1fd40fac26fc85968f0106269d2bcd94937a7c32ee98c33",
			"dd6d9efff5004b007dcb2d5992cfdc5bc74428820ef40c66db1454a13826451b",
			"f5df84a6499660de837b5e5bd4734cc32077a56b68c1759d9ab3de10cc532bc6"
		],
		merkle_root: "990e01e3959627d2ddd94927e1c605a422b62dc3b8c8b98d713ae6833c3ef122",

	   },
	   BenchmarkSample {
		icon_address : "2ddfa6d1cdf98944a90319bbe57c10d6a3527195",
		icon_signature : "dd0fc37634a650ee0d13f848ea3a2f62b9d80520ae5ec8faaaee662aa6170bfb4e34a735557447dd226ef4e9793a8519ee2f04fdbe6024fb29bcd4c95edcab6900",
		message : "icx_sendTransaction.data.{method.transfer.params.{wallet.1280a5490ab78c44127424fa8f846b1e1459caf5405e17f66cc7ca642e651370}}.dataType.call.from.hx2ddfa6d1cdf98944a90319bbe57c10d6a3527195.nid.0x1.nonce.0x1.stepLimit.0x0.timestamp.0x0.to.hx2ddfa6d1cdf98944a90319bbe57c10d6a3527195.version.0x3",
		ice_address : "1280a5490ab78c44127424fa8f846b1e1459caf5405e17f66cc7ca642e651370",
		ice_signature : "ce9100b0ffaa9277de4898fbe5a31f64a5e43d5319674585838dd6804ac3d555560b68e10ab501ceb1f1247fb199b21e293d9f21f72f972ec70eaa6351a4df84",
		amount : 2000000000000000000,
		defi_user : true,
		merkle_proofs : &[
			"f0581f454bb54b31bb8cb7ee3b8196d438a8588835f9f15328d09b30e35aba44",
			"76165bcd9d4384607dec75039fda663be627ad42eb722bb8681a800244241d64",
			"46e617f4ee231a73f2bf6963eb4f044b77c956b95b20201e89fd495c600270c4",
			"2b003ed0aa2100689bc76a0940745c7aa244f89a3cfa7a80872d6169e092067b",
			"6045f8c48195fb8ca809b197e3605dc4767f34df0b30094645fd6f92e9063076",
			"7ac43c63fb6b2d1576f99004881ec01dc6edc2015e972d837ab85f482f2ea318",
			"dd6d9efff5004b007dcb2d5992cfdc5bc74428820ef40c66db1454a13826451b",
			"f5df84a6499660de837b5e5bd4734cc32077a56b68c1759d9ab3de10cc532bc6"
		],
		merkle_root:"990e01e3959627d2ddd94927e1c605a422b62dc3b8c8b98d713ae6833c3ef122",

	},
	BenchmarkSample{
		icon_address: "bf721af547f9b594b00fd675d66f267fead26078",
		icon_signature: "6f72df8c2137f5a72cb6b7f72536c58da0fbe355c34f4eb7ea4d361e44c2846e2c29c10dd30b92d097c50f223373e4d9dc346b290d6e27077cdf9a5c84e0e01901",
		message: "icx_sendTransaction.data.{method.transfer.params.{wallet.a2e3b45a628e579fba6398bd61b25e237ab0cdd18190127e1b75929bb190e303}}.dataType.call.from.hxbf721af547f9b594b00fd675d66f267fead26078.nid.0x1.nonce.0x1.stepLimit.0x0.timestamp.0x0.to.hxbf721af547f9b594b00fd675d66f267fead26078.version.0x3",
		ice_address: "a2e3b45a628e579fba6398bd61b25e237ab0cdd18190127e1b75929bb190e303",
		ice_signature: "ecf730df0b51b12257d2c2d86945b5df2ca7076ffd448d050d9819786b63dd29276a2ba4757b611bdbf5e9f51dcba6122f57a3cd95cffd1b42197a8295e2ad82",
		amount: 3000000000000000000,
		defi_user: true,
		merkle_proofs: &[
			"798f62d51e94cb89df6260d60aa48baeef1c5a99988608e9e8cd8dbd5a8cf895",
			"e93d3837fcc639d1f9abc176e0d92237c5b0b0e500738d6e5b35e4006272c366",
			"97d8d7ec83be95c41e4f80c27c29fe851613af01c49eb0d19b216839ffc1fd63",
			"f81ee4d3397af15073ba83cbf5a640f98ea7fd48596876795731ce6aac6b09d0",
			"57a0ac7ad8d609c2ffa4fc0755b4001a8d9902c8237d8912b5645aeae18f00b5",
			"1fe97a9f0936a47663102bc1e7f462be70166ec7865ee52abb8b0f8e96ac2532",
			"c42ffc995410faf61588017be9c7d96716dfe8d5758cc0c47faaae84131cd152",
			"9a76bf1c50b1699066273fd5637ef2d80d507bbf19d7b41684768e92e6b0faac",
			"16007529c0508b834034a7ec9a27026a596cc3dbbdbc817c23cdcab944f706c2",
			"c8c790ef89aec760e0087d79668d5f25bfcfbba74865bc2f514e259f1607015b"
		],
		merkle_root:"990e01e3959627d2ddd94927e1c605a422b62dc3b8c8b98d713ae6833c3ef122",

	},
	BenchmarkSample{
		icon_address : "669fe1b8d281899304a10bcc43c008b80c2a832d",
		icon_signature : "49e476b6c8d9ea10a318a3a92b3cb2d69636c5f2717f9b7300838c9cf530f21a3436d31c96db65b7a10599c3b149df6a6759574b23b12718e7ce481c0963a61400",
		message : "icx_sendTransaction.data.{method.transfer.params.{wallet.6ca3c5c62fe77c4c46cf9658891cf13bd4cf46ab6615273f6321701edba96c51}}.dataType.call.from.hx669fe1b8d281899304a10bcc43c008b80c2a832d.nid.0x1.nonce.0x1.stepLimit.0x0.timestamp.0x0.to.hx669fe1b8d281899304a10bcc43c008b80c2a832d.version.0x3",
		ice_address : "6ca3c5c62fe77c4c46cf9658891cf13bd4cf46ab6615273f6321701edba96c51",
		ice_signature : "aa1bf82922fcf897249ec77ef46b43b82f126947baebea2b20653f9b62910915d581e5ac2c134b12b515cac2e85dd1bb96573ba67311b0f711d6ca18e2e3d68a",
		amount : 4000000000000000000,
		defi_user : true,
		merkle_proofs : &[
			"4cb4baa6637f5d39eae5d3998b35a15d8ff245e006e0c3c354e1cf641bf89894",
			"44912681ae011b28768811e9ab063f4a9988580c82f2a71122cd719f922ef3ee",
			"bc3957fa95f3ccba5a7f626b65cd4b9f8c6512ec67dd281feea1738bb1202dc3",
			"0c5721659c64f2c1da34d4f8cdcaa7d770860e6ad774cd9d5c9f4e4fd9d16d06",
			"e784711156cde55af3f71429d64e31062233e3dc2f98712425e0854f3b261ccd",
			"89ff5572284e1c8f4fdc43c9499d75addcedd53a79d4dcf79dcc3f0b48263d5c",
			"5cd8cb26c4b08ade5da89fe0e3dce3d7a5c5d039364ebc5ae4ccf628aa7d7973",
			"852ccbd392aef04c99b16c1441c8fa690183c635e9cd1534c21c13f4ce3d505b",
			"348af565a5d5b2a4750c6c6fb9b272f0331eb001a70abb4d414bbd90eff2aa56",
			"c8c790ef89aec760e0087d79668d5f25bfcfbba74865bc2f514e259f1607015b"
		],
		merkle_root:"990e01e3959627d2ddd94927e1c605a422b62dc3b8c8b98d713ae6833c3ef122",

	}
   ];

const creditor_key: sp_core::sr25519::Public = sr25519::Public([1; 32]);

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	set_airdrop_server_account {
				let old_account: types::AccountIdOf<T> = frame_benchmarking::whitelisted_caller();

				let new_account: types::AccountIdOf<T> = frame_benchmarking::whitelisted_caller();

				<ServerAccount<T>>::set(Some(old_account.clone()));

			}: set_airdrop_server_account(RawOrigin::Root,new_account.clone())
	verify {
				assert_last_event::<T>(Event::ServerAccountChanged{
					old_account:Some(old_account.clone()),
					new_account:new_account.clone()
				}.into());
	}

	update_airdrop_state {
		let old_state= Pallet::<T>::get_airdrop_state();
		let new_state = types::AirdropState::default();

	}: update_airdrop_state(RawOrigin::Root, new_state.clone())
	verify {
		 assert_last_event::<T>(Event::AirdropStateUpdated {
			old_state,
			new_state,
		}.into());
	}


	dispatch_user_claim {
		let x in 0 .. 3;
		let caller: types::AccountIdOf<T> = frame_benchmarking::whitelisted_caller();
		// let ofw_account = sr25519::Public([1; 32]).into_account();
		Pallet::<T>::set_creditor_account(creditor_key);
		let system_account_id = Pallet::<T>::get_creditor_account().unwrap();
		Pallet::<T>::init_balance(&system_account_id,10_000_000_000_000_000_000_000_000);
		let case= UserClaimTestCase::<<T as pallet::Config>::MaxProofSize>::try_from(benchmark_samples[x as usize].clone()).unwrap();
		let amount = <T::BalanceTypeConversion as Convert<_, _>>::convert(case.amount);
		 let icon_address=case.icon_address.clone();
		 let mut new_state = types::AirdropState::default();
		 new_state.block_claim_request=false;
		 new_state.block_exchange_request=false;
		<AirdropChainState<T>>::set(new_state.clone());

	}: dispatch_user_claim(
		RawOrigin::Root,
		case.icon_address,
		case.ice_address,
		case.message,
		case.icon_signature,
		case.ice_signature,
		amount,
		case.defi_user,
		case.merkle_proofs)
	verify {
		assert_last_event::<T>(Event::ClaimSuccess(icon_address.clone()).into());
	}

	dispatch_exchange_claim {
		let x in 0 .. 3;

		Pallet::<T>::set_creditor_account(creditor_key);
		let system_account_id = Pallet::<T>::get_creditor_account().unwrap();
		Pallet::<T>::init_balance(&system_account_id,10_000_000_000_000_000_000_000_000);
		let case= UserClaimTestCase::<<T as pallet::Config>::MaxProofSize>::try_from(benchmark_samples[x as usize].clone()).unwrap();
		let amount = <T::BalanceTypeConversion as Convert<_, _>>::convert(case.amount);
		let icon_address=case.icon_address.clone();
		<ExchangeAccountsMap<T>>::insert(icon_address.clone(),amount);
		let mut new_state = types::AirdropState::default();
		new_state.block_claim_request=false;
		new_state.block_exchange_request=false;
		<AirdropChainState<T>>::set(new_state.clone());


	}: dispatch_exchange_claim(
		RawOrigin::Root,
		icon_address.clone(),
		case.ice_address,
		amount,
		case.defi_user,
		case.merkle_proofs)
	verify {
		assert_last_event::<T>(Event::ClaimSuccess(icon_address.clone()).into());
	}

	change_merkle_root {
		let p in 0..10;
		let new_root = [p as u8;32];
		let last_root = [0u8;32];
		MerkleRoot::<T>::put(last_root.clone());
	}: change_merkle_root(
		RawOrigin::Root,
		new_root
	) verify {
		assert_last_event::<T>(Event::MerkleRootUpdated{
			old_root: Some(last_root),
			new_root,
		}.into());
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
