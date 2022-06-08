<div align="center">

![ice.png](docs/media/ice.png)

[![GitHub last commit](https://img.shields.io/github/last-commit/web3labs/ice-substrate)](https://github.com/web3labs/ice-substrate/commits/main)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/web3labs/ice-substrate)](https://github.com/web3labs/ice-substrate/tags)
![license](https://img.shields.io/badge/License-Apache%202.0-blue?logo=apache&style=flat-square)
[![Twitter URL](https://img.shields.io/twitter/follow/icenetwork_io?style=social)](https://twitter.com/icenetwork_io)
[![Medium](https://img.shields.io/badge/Medium-gray?logo=medium)](https://medium.com/@helloiconworld)
[![Telegram](https://img.shields.io/badge/Telegram-gray?logo=telegram)](https://t.me/joinchat/UG3uX-USLBwxYWRh)

</div>
ICE Network is an EVM compatible network built with Parity’s Substrate framework. ICE is the first network to use the Substrate SDK to extend the feature-set of an existing layer one blockchain protocol. In addition, ICE will provide the much-needed addition of EVM compatibility to the ICON ecosystem

## Setup

Install Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Nightly build and toolchain setup

```
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

This should be sufficient to get started, refer to [Substrate installation](https://docs.substrate.io/v3/getting-started/installation/) guide for more details if needed

## Build
Prerequisites (Ubuntu 20.04)

1. rust toolchain with `wasm32-unknown-unknown` target added
2. `build-essential`
3. `libclang-dev`

To build the chain, execute the following commands from the project root:

```
$ cargo build --release
```
### Start Frost Node (Development)

To execute the chain, run:

```
$ ./target/release/ice-node --dev --tmp
```

**Note**: optional `--tmp` flag will automatically purge the node DB

This will start a local development ICE node with solo validator and predefined accounts, use [Polkadot-JS](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944) to explore the network 

Use manual seal to produce block manually through RPC  

```
$ ./target/release/ice-node --dev --sealing manual
```

### Start Arctic Collator (Local Relay Chain)

```
git clone https://github.com/paritytech/polkadot.git -b release-v0.9.19
cd polkadot
cargo build --release
cd -
```

**Install Polkadot-launch**

Prerequisites (tested on Ubuntu 20.04)

1. node version 16.2.0
2. yarn (using `npm install --global yarn`)
```
yarn global add polkadot-launch
```

**Run Polkadot-launch**

```
cd ./ice-substrate/resources

polkadot-launch arctic-launch.json
```

It will about 2 minutes to start the relay chain and arctic collator
