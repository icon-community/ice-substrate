#!/bin/bash
../polkadot/target/release/polkadot \
--bob \
--validator \
--base-path /tmp/relay/bob \
--chain rococo-local.json \
--port 30334 \
--ws-port 9945