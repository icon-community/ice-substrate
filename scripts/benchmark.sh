RUSTC_WRAPPER=/Users/abhi/.cargo/bin/sccache cargo build --features runtime-benchmarks --release
declare -a arr=(
"frame_system" 
"pallet_assets" 
"pallet_authorship" 
"pallet_balances" 
"pallet_base_fee" 
"pallet_bounties" 
"pallet_collective" 
"pallet_contracts" 
"pallet_democracy" 
"pallet_dynamic_fee" 
"pallet_elections_phragmen" 
"pallet_evm" 
"pallet_fees_split" 
"pallet_grandpa" 
"pallet_identity" 
"pallet_indices" 
"pallet_membership" 
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
"pallet_vesting" 
"pallet_airdrop" 
"cumulus_pallet_xcmp_queue" 
)

## now loop through the above array
chain="${1:-dev}"
folder="frost"
if [ $1 = "snow-kusama" ]; then
  folder="snow"
elif [ $1 = "arctic" ]; then
  folder="arctic"
else
  folder="frost"
fi

for i in "${arr[@]}"
do
  ./target/release/ice-node benchmark pallet \
    --chain "$chain" \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet "$i" \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output runtime/"$folder"/src/weights/"$i"_weight.rs
  done
