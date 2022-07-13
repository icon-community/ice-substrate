#!/bin/bash
# Start Relay `Alice` node
../polkadot/target/release/polkadot \
--alice \
--validator \
--base-path /tmp/relay/alice \
--chain rococo-local.json \
--port 30333 \
--ws-port 9955 