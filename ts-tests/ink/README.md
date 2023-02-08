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
- On local machine, create `.env.local` file with reference from `.env.sample` file and run 
  ```sh
  npm run test
  ```
- On server, set the env vars accordingly and run
  ```sh
  npm run test:server
  ```
- **Optional:** *Update `constants.ts` file for changing RPC endpoints*

## Other Commands
- Deploy a contract on mainnet to test state after upgrade
  ```sh
  npm run deploy-contract
  ```
- Ensure the contract state is intact after upgrade
  ```sh
  npm run test-contract-state
  ```
- - Make sure contracts can be deployed, queried and called after upgrade
  ```sh
  npm run test-contract-overall
  ```
