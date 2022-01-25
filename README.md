
![license](https://img.shields.io/badge/License-Apache%202.0-blue?logo=apache&style=flat-square)

[![Twitter URL](https://img.shields.io/twitter/follow/icenetwork_io?style=social)](https://twitter.com/icenetwork_io)
[![Medium](https://img.shields.io/badge/Medium-gray?logo=medium)](https://medium.com/@helloiconworld)
[![Telegram](https://img.shields.io/badge/Telegram-gray?logo=telegram)](https://t.me/joinchat/UG3uX-USLBwxYWRh)

ICE Network is an EVM compatible network built with Parity’s Substrate framework.

## Build & Run

To build the chain, execute the following commands from the project root:

```
$ cargo build --release
```

To execute the chain, run:

```
$ ./target/release/ice-node --dev
```

The node also supports to use manual seal (to produce block manually through RPC).  
This is also used by the ts-tests:

```
$ ./target/release/ice-node --dev --manual-seal
```
