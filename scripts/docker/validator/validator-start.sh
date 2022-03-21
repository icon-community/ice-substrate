#!/bin/bash
cd /root/ice-node/
chmod -Rf 777 ice-node
chown -Rf root.root ice-node
./ice-node build-spec --disable-default-bootnode --chain test > /root/ice-node/frost_testnet.json
./ice-node build-spec --chain=/root/ice-node/frost_testnet.json --raw --disable-default-bootnode > /root/ice-node/frost_testnetRaw.json
./ice-node key insert --base-path /data/ice_node_test/ --chain /root/ice-node/frost_testnetRaw.json --scheme Sr25519 --suri 0x20f6fc98aa2806d373fcdd34f7644e9327b81497eefeeadb44c62873ec1e7c9a --password-filename /root/ice-node/keystore.txt --key-type aura
./ice-node key insert --base-path /data/ice_node_test/ --chain /root/ice-node/frost_testnetRaw.json --scheme Ed25519 --suri 0x20f6fc98aa2806d373fcdd34f7644e9327b81497eefeeadb44c62873ec1e7c9a --password-filename /root/ice-node/keystore.txt --key-type gran
./ice-node --base-path /data/ice_node_test/ --chain ./frost_testnetRaw.json --port 30333 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator --name FrostValidator1_DEV --bootnodes /ip4/172.19.0.10/tcp/30333/p2p/$1
