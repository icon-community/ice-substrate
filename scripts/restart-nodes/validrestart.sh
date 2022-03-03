#!/bin/bash
cd /root/ice-node/
systemctl stop validator.service
chmod -Rf 777 ice-node
chown -Rf root.root ice-node
#./ice-node purge-chain --base-path /tmp/ice_node_test/ --chain test -y
rm -rvf /tmp/ice_node_test/chains/testnet/keystore/6*
./ice-node key insert --base-path /tmp/ice_node_test/ --chain /root/ice-node/frost_testnetRaw.json --scheme Sr25519 --suri 0x20f6fc98aa2806d373fcdd34f7644e9327b81497eefeeadb44c62873ec1e7c9a --password-filename /root/ice-node/password.txt --key-type aura
./ice-node key insert --base-path /tmp/ice_node_test/ --chain /root/ice-node/frost_testnetRaw.json --scheme Ed25519 --suri 0x20f6fc98aa2806d373fcdd34f7644e9327b81497eefeeadb44c62873ec1e7c9a --password-filename /root/ice-node/password.txt --key-type gran
systemctl restart validator.service
sleep 8
status=$(systemctl status validator.service)
echo $status

