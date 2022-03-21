#!/bin/bash
cd /root/ice-node/
chmod -Rf 777 ice-node
chown -Rf root.root ice-node
./ice-node build-spec --disable-default-bootnode --chain test > /root/ice-node/frost_testnet.json
./ice-node build-spec --chain=/root/ice-node/frost_testnet.json --raw --disable-default-bootnode > /root/ice-node/frost_testnetRaw.json
./ice-node key insert --base-path /data/ice_node_test --chain /root/ice-node/frost_testnetRaw.json --scheme Sr25519 --suri 0x605f45a75c60a85b859454dcd43ce124dd1e6f1b1e727f2f5288be3422853580 --password-filename /root/ice-node/keystore.txt --key-type aura
./ice-node key insert --base-path /data/ice_node_test --chain /root/ice-node/frost_testnetRaw.json --scheme Ed25519 --suri 0x605f45a75c60a85b859454dcd43ce124dd1e6f1b1e727f2f5288be3422853580 --password-filename /root/ice-node/keystore.txt --key-type gran
./ice-node --base-path /data/ice_node_test/ --chain ./frost_testnetRaw.json --port 30333 --ws-external --ws-port 9945 --rpc-external --rpc-port 9933 --rpc-cors all --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --rpc-methods Unsafe --validator --name DEVFrostRPC
