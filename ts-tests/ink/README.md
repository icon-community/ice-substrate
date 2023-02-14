<div align="center">

<h1 align="center">
    Tests for ink smart contracts on ICE/SNOW Network
</h1>

[![Author](https://img.shields.io/badge/author-@sharma66mahesh-blue.svg?style=flat-square)](https://github.com/sharma66mahesh)
[![Maintainer](https://img.shields.io/badge/maintainer-@SNOWnetwork-blue.svg?style=flat-square)](https://github.com/web3labs/ice-substrate)
[![Test Status](https://github.com/ibriz/ice-ink-contracts-tests/actions/workflows/mocha-tests.yml/badge.svg)
</div>

## How to Run
- Tested with Node `v16.14.0`
- Build contracts on `assets` folder.
- **Optional:** *Update contract metadata and wasm paths on `constants.ts` file.* 
- Install node dependencies
  ```sh
  npm i
  ```
- On local machine, create `.env.local` file with reference from `.env.sample` file and run 
  ```sh
  npm run test
  ```
- On server, set the env vars accordingly and run
  ```sh
  npm run test:server
  ```
- **Optional:** *Update `constants.ts` file for changing RPC endpoints*

## Run tests for contracts state
- Ensure the contract state is intact after upgrade
  ```bash
  npm run test-ctx-state <chain>
  ```
  *where `chain` could be one of the following:*  
  - *snow*
  - *arctic*
  - *snow_staging*
  - *local*  

- `The contract should be readily available on SNOW/Arctic/Staging Network.`  
*<b>Optionally:</b>* If you save your `private key` in `.env` file, you can deploy the contract to SNOW/Arctic:
  ```bash
  npm run deploy-upgrade-ctx <chain>
  ```
  *where `chain` could be one of the following:*  
  - *snow*
  - *arctic*
  - *snow_staging*
  - *local*

- Make sure contracts can be deployed, queried and called after upgrade
  ```sh
  npm run test-ctx-overall <chain>
  ```
    *where `chain` could be one of the following:*  
  - *snow*
  - *arctic*
  - *snow_staging*
  - *local*
