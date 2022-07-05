use crate::types;
use crate::Config;
use codec::alloc::string::ToString;
use core::cmp::Ordering;
use core::marker::PhantomData;
use sp_io::hashing;
use sp_std::prelude::*;
use types::MerkelProofValidator;

pub struct AirdropMerkleValidator<T>(PhantomData<T>);

impl<T: Config> MerkelProofValidator<T> for AirdropMerkleValidator<T> {
	fn validate(
		leaf_hash: types::MerkleHash,
		root_hash: types::MerkleHash,
		proofs: types::MerkleProofs<T>,
	) -> bool {
		let computed_root = hex::encode(proof_root(leaf_hash, proofs.to_vec()));
		let root_hex = hex::encode(root_hash);

		computed_root == root_hex
	}
}

pub fn hash_leaf(
	icon_address: &types::IconAddress,
	amount: types::ServerBalance,
	defi_user: bool,
) -> [u8; 32] {
	let defi_str = if defi_user { "1" } else { "0" };
	let mut byte_vec = icon_address.to_vec();
	byte_vec.extend_from_slice(amount.to_string().as_bytes());
	byte_vec.extend_from_slice(defi_str.as_bytes());
	hashing::blake2_256(&byte_vec)
}

pub fn proof_root(leaf_hash: types::MerkleHash, proofs: Vec<types::MerkleHash>) -> [u8; 32] {
	let mut one = leaf_hash;
	for proof in proofs {
		one = create_hash(one, proof);
	}

	one
}

pub fn create_hash(one: types::MerkleHash, other: types::MerkleHash) -> [u8; 32] {
	let sorted = sort_array(one, other, 0_usize);
	hashing::blake2_256(&sorted)
}

pub fn sort_array(one: types::MerkleHash, other: types::MerkleHash, pos: usize) -> Vec<u8> {
	let max_pos = 31_usize;
	let mut pos = pos;
	let ord = one[pos].cmp(&other[pos]);
	match ord {
		Ordering::Greater => [other, one].concat(),
		Ordering::Less => [one, other].concat(),
		Ordering::Equal => {
			if pos == max_pos {
				return [one, other].concat();
			}
			pos += 1;
			sort_array(one, other, pos)
		}
	}
}
