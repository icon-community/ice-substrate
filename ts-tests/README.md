# Functional testing for Substrate Ice Node RPC

This folder contains a set of functional tests designed to perform functional testing on the Ice Eth RPC.

It is written in typescript, using Mocha/Chai as Test framework.

## Test flow

Tests are separated depending on their genesis requirements.
Each group will start a `Ice test node` with a given `spec` before executing the tests.

## Build the manual seal node for tests

```bash
cargo build --release --no-default-features --features manual-seal,rpc_binary_search_estimate
```

## Installation

```bash
npm install
```

## Run automated tests

```bash
npm run build && npm run test
```

You can also add the Ice Node logs to the output using the `ICE_LOG` env variable. Ex:

```bash
ICE_LOG="warn,rpc=trace" npm run test
```

(The Ice node be listening for RPC on port 19933, mostly to avoid conflict with already running substrate node)

## Run tests for contracts state
Ensure that the deployed contract state is intact.
```bash
npm run test-ctx-state <chain>
```
*where `chain` could be one of the following:*  
- *snow*
- *arctic*
- *snow_staging*
- *local*  

`The contract should be readily available on SNOW/Arctic/Staging Network.`  
*<b>Optionally:</b>* If you save your `private key` in `.env` file, you can deploy the contract to SNOW/Arctic:
```bash
npm run deploy-upgrade-ctx <chain>
```
*where `chain` could be one of the following:*  
- *snow*
- *arctic*
- *snow_staging*
- *local*