use crate::merkle::{hash_leaf, proof_root, sort_array};
use crate::utils;
use hex_literal;

#[test]
fn test_hash_leaf() {
	let expected = "7fe522d63ebcabfa052eec3647366138c23c9870995f4af94d9b22b8c5923f49";
	let icon_addr: [u8; 20] = hex_literal::hex!("a99344ea068864f8af6cbcf89328d6eb3d7e8c9c");
	let result = hex::encode(hash_leaf(&icon_addr, 0, true));
	assert_eq!(expected, &result);
}

#[test]
fn test_verify_proof() {
	let root = "0ad37ff10c4e2f80b4f66c376077e664e5333fd6e256385cf7ff2b03952bb2e2";
	let cases = [
		(
			"7fe522d63ebcabfa052eec3647366138c23c9870995f4af94d9b22b8c5923f49",
			[
				"813340daefd7f1ca705faf8318cf6455632259d113c06e97b70eeeccd43519a9",
				"409519ab7129397bdc895e4da05871c9725697a5e092addf2fe90f6e795feb8f",
				"38055bb872670c69ac3461707f8c0b4b8e436eecfc84cfd80db30db3030c489a",
			]
			.to_vec(),
		),
		(
			"c0401b78aed1385426cf3edba8f8b2d25f9fae0f26883fe9b72cf9b4f2d121f0",
			[
				"be77163fe3d25465685bb3d8004b7a8b6a260906b9d4e5fa49427c2f1b789f10",
				"15defb2be27c700d7e3e673652a8dd7609c6d6f84f4dd007880884e0701bcb38",
			]
			.to_vec(),
		),
		(
			"be77163fe3d25465685bb3d8004b7a8b6a260906b9d4e5fa49427c2f1b789f10",
			[
				"c0401b78aed1385426cf3edba8f8b2d25f9fae0f26883fe9b72cf9b4f2d121f0",
				"15defb2be27c700d7e3e673652a8dd7609c6d6f84f4dd007880884e0701bcb38",
			]
			.to_vec(),
		),
		(
			"23ac30dcdf69edb2f243b94608340ba3164424c24f9b5c0f56959b737327b515",
			[
				"4b4bb156d99b6d40e8edfa3c0d50fc4296452276b5cd4f936bceddee37c0505d",
				"d7608226420bd49c0d8e5f79a58c5d693341f2c299911abc1eb96665b85e551d",
				"38055bb872670c69ac3461707f8c0b4b8e436eecfc84cfd80db30db3030c489a",
			]
			.to_vec(),
		),
	];
	for case in cases {
		verify_proof_case(root, case.0, case.1);
	}
}

#[test]
fn test_verify_proof_real() {
	let root = "4c59b428da385567a6d42ee1881ecbe43cf30bf8c4499887b7c6f689d23d4672";
	let cases = [
		(
			"e56abc4f8308afe283b128761e3d00202aa3fa502526b317b37e114a5d52416c",
			[
				"e5c1cefc1be025a1f2851d1c21a1e4417fb89a8ac8a10d3655c915a514160c9d",
				"beab672f1c4ec3b1851d054f8e9ecae5f45646d3c67398174cd4c6d0ae9e15d6",
				"92379acb8de9c3999d5e34bb0c280ba3a7ea8a7fb92474099b9f05c1e5856b57",
				"3ad35d31ebe6d14143f2401f2fa9d5816dc5cfb12c9041afbf1f653a381ba289",
				"7af4325e3a2a279b581ba9877800c1ef77e9f8208b8750b3dd46c7848f337491",
				"c2d3826ca7c9ed3c53e4757e5b222083f985f07109c800d5e4090dca8cb94779",
				"5e23727864ebad198df2f497cb3e74455f280dad3c92c3b368bddd91dafd48ec",
				"703ad3a90417ac11be663bd45122ef7bb139e2ccde51889c1c3922c93e78f8b7",
				"335fb756d0b76f5ddba80a4238ad8b19e2fda8e620de8fc8eeabef08e53c8c1c",
				"ab12930064ec318bb8501d88670d659da956a1cdb30cbf7033f561d6368c40e9",
			]
			.to_vec(),
		),
		(
			"80e0c54fffa1fccedec918897b988e5a396cf86afa9d56f86f3e75cf5dd6beb8",
			[
				"80f607b0d69e05f187a1c481cdb7f40a2d8d9d23e694c96486a80dcc5b15e8d7",
				"61a8a6ccc07f7cc8563892120a0a818475055db01c221b1f45369f28ab4fde4a",
				"e71a091b1e8ad13a511d4fee208b2c0cc80e9fedd457c400fdd9de863fd1cbae",
				"679566d212dc6e753d3aa25fbd31620a8bc6f0632c58583d9ffbccb9234d899f",
				"5c0fe24772fa2c22212448a70cd086901ae9bfe23c125adad9363705a24aa21f",
				"84773b8829d2ee30a7a9e3284545585a68f6a55a0327c78c6cc8a32c43bc54b1",
				"8b51868abe71d47d25c0320f6f20fc98f0d2c871de458eb1c27d0a35025dda1d",
				"119ad5bf90649c0facab1b8ac79aa6b7f86e9d4f515416973c3a369c5159496f",
				"b7eadaab875f75c76e53d097e843d7c1bdb3588d039dfd8135b881c531ff81ea",
				"fb8164dab56af7180161c00e3f52d149cdc965d431f6b4c4a4900d70d51cc07d",
			]
			.to_vec(),
		),
		(
			"59c897680f5bf10d7ee3220b09122850366382ebbf58a64d0b5fc30980050635",
			[
				"5a27f7e3e8d8b5318af373d069b9d18a872fc9985ed366ea24f690b8cdbdbf95",
				"af696a54a2127c55df65e32b0750b499460045b2ff6615b9da78ad58283357e2",
				"bf2f0e4eaadf121e71ce8bbb7ee9960fe5212dfb84468f884a8e4e57e61e7283",
				"8d75b5b1270191fd911374baed4bd6e1f6d9781f2707ba95f13d1fb704dd9184",
				"1cff6a1bd58b0cabb43e0259301b894483244df086c369c7f86d8e798af5fe5f",
				"df249d276430cc76524bfa35ece6407f9c6c8f4a1f5ac65b8702ce295f9014df",
				"89a56eaeb1d15212ba14e0a9e1dd2e2eed26f3667de97b5ae9147678dd909ff4",
				"cb12817002e9fb84b284efdcf8003eede561bacf8fe9fe452e37c9156a638ba6",
				"b7eadaab875f75c76e53d097e843d7c1bdb3588d039dfd8135b881c531ff81ea",
				"fb8164dab56af7180161c00e3f52d149cdc965d431f6b4c4a4900d70d51cc07d",
			]
			.to_vec(),
		),
		(
			"6957d6b1ff44e34e3a5135d22d22de1b13a1be3879d20a82309a6053175cbe45",
			[
				"68ce45479b6abe7f059e44e0d1b2377de9eae5a5e58d7346f61aedc14228ca5d",
				"88998e74d649a12eccffbd30c3ea41565888b341a32ee608a89231d055404797",
				"75f735e7b39986b5a61794f4e25d25d7b3874ea49e23436a9bf242a1dd7bf2e6",
				"db83f23d14ab7e9369147b7d4148a23156de1a9cc82695647821c6ea50006c85",
				"f5f2b94cbc67bd0703c55380a14794d5703783fbcd793aa7a14f86d06995039c",
				"1f46436e7b14ee57da20425a6bcc5485affc02f5839744922e713c26f6c1066b",
				"efc70368687b58f487cc3369794a4d5265c42be00186d51e610d8d987e963b81",
				"119ad5bf90649c0facab1b8ac79aa6b7f86e9d4f515416973c3a369c5159496f",
				"b7eadaab875f75c76e53d097e843d7c1bdb3588d039dfd8135b881c531ff81ea",
				"fb8164dab56af7180161c00e3f52d149cdc965d431f6b4c4a4900d70d51cc07d",
			]
			.to_vec(),
		),
	];
	for case in cases {
		verify_proof_case(root, case.0, case.1);
	}
}

#[test]
fn fails_invalid_proof() {
	let root = "4c59b428da385567a6d42ee1881ecbe43cf30bf8c4499887b7c6f689d23d4672";
	let invalid_leaf = "e56abc4f8308afe283b128761e3d00202aa3fa502526b317b37e114a5d52416e";
	let leaf_hash = utils::hex_as_byte_array(invalid_leaf).unwrap();
	let proofs = [
		"e5c1cefc1be025a1f2851d1c21a1e4417fb89a8ac8a10d3655c915a514160c9d",
		"beab672f1c4ec3b1851d054f8e9ecae5f45646d3c67398174cd4c6d0ae9e15d6",
		"92379acb8de9c3999d5e34bb0c280ba3a7ea8a7fb92474099b9f05c1e5856b57",
		"3ad35d31ebe6d14143f2401f2fa9d5816dc5cfb12c9041afbf1f653a381ba289",
		"7af4325e3a2a279b581ba9877800c1ef77e9f8208b8750b3dd46c7848f337491",
		"c2d3826ca7c9ed3c53e4757e5b222083f985f07109c800d5e4090dca8cb94779",
		"5e23727864ebad198df2f497cb3e74455f280dad3c92c3b368bddd91dafd48ec",
		"703ad3a90417ac11be663bd45122ef7bb139e2ccde51889c1c3922c93e78f8b7",
		"335fb756d0b76f5ddba80a4238ad8b19e2fda8e620de8fc8eeabef08e53c8c1c",
		"ab12930064ec318bb8501d88670d659da956a1cdb30cbf7033f561d6368c40e9",
	]
	.into_iter()
	.map(|h| {
		let bytes: [u8; 32] = utils::hex_as_byte_array(h).unwrap();
		bytes
	})
	.collect::<Vec<[u8; 32]>>();
	let proof_root = proof_root(leaf_hash, proofs);
	assert_ne!(root, hex::encode(proof_root));
}

#[test]
fn test_sort_array() {
	let arr1 = [0u8; 32];
	let arr2 = [0u8; 32];
	let result = sort_array(arr1, arr2, 0 as usize);
	assert_eq!(result, [arr1, arr2].concat());

	let arr1 = [0u8; 32];
	let arr2 = [1u8; 32];
	let result = sort_array(arr1, arr2, 0 as usize);
	assert_eq!(result, [arr1, arr2].concat());

	let arr1 = [2u8; 32];
	let arr2 = [0u8; 32];
	let result = sort_array(arr1, arr2, 0 as usize);
	assert_eq!(result, [arr2, arr1].concat());
}

pub fn verify_proof_case(root: &str, leaf: &str, proofs: Vec<&str>) {
	let leaf_hash = utils::hex_as_byte_array(leaf).unwrap();

	let proofs = proofs
		.into_iter()
		.map(|h| {
			let bytes = utils::hex_as_byte_array(h).unwrap();
			bytes
		})
		.collect::<Vec<[u8; 32]>>();
	let proof_root = proof_root(leaf_hash, proofs);
	assert_eq!(root, hex::encode(proof_root));
}
