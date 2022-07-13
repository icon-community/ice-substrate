#!/bin/bash

./target/release/ice-node \
--collator \
--force-authoring \
--chain snow-raw.json \
--base-path /tmp/snow \
--port 40333 \
--ws-port 9944 \
--rpc-cors all \
-- \
--execution wasm \
--chain rococo-local.json \
--port 30343 \
--ws-port 9977