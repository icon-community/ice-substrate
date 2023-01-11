cargo build --features runtime-benchmarks --release
declare -a arr=("pallet_airdrop" 
		 "pallet_assets" 
		 "pallet_authorship" 
		 "pallet_balances" 
		 "pallet_base_fee" 
		 "pallet_bounties" 
		 "pallet_contracts" 
		 "pallet_democracy" 
		 "pallet_dynamic_fee" 
		 "pallet_ethereum" 
		 "pallet_evm" 
		 "pallet_fees_split" 
		 "pallet_grandpa" 
		 "pallet_identity" 
		 "pallet_indices" 
		 "pallet_multisig" 
		 "pallet_preimage" 
		 "pallet_proxy" 
		 "pallet_randomness_collective_flip" 
		 "pallet_scheduler" 
		 "pallet_session" 
		 "pallet_simple_inflation" 
		 "pallet_sudo" 
		 "pallet_timestamp" 
		 "pallet_tips" 
		 "pallet_transaction_payment" 
		 "pallet_treasury" 
		 "pallet_utility" 
		 "pallet_vesting" )

## now loop through the above array
for i in "${arr[@]}"
do
  ./target/release/ice-node benchmark pallet \
    --chain dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet "$i" \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output runtime/common/src/weights/"$i"_weight.rs
done