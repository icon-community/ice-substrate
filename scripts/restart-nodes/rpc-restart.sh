#!/bin/bash
cd /root/ice-node/
systemctl stop rpc.service
chmod -Rf 777 ice-node
chown -Rf root.root ice-node
#./ice-node purge-chain --base-path /tmp/ice_node_test/ --chain $1 -y
rm -rvf /root/ice-node/frost*
rm -rvf /tmp/ice_node_test/chains/testnet/keystore/6*
./ice-node build-spec --disable-default-bootnode --chain $1 > /root/ice-node/frost_testnet.json
./ice-node build-spec --chain=/root/ice-node/frost_testnet.json --raw --disable-default-bootnode > /root/ice-node/frost_testnetRaw.json
scp frost_testnetRaw.json root@51.159.130.133:/root/ice-node/
./ice-node key insert --base-path /tmp/ice_node_test --chain /root/ice-node/frost_testnetRaw.json --scheme Sr25519 --suri 0x605f45a75c60a85b859454dcd43ce124dd1e6f1b1e727f2f5288be3422853580 --password-filename /root/ice-node/password.txt --key-type aura
./ice-node key insert --base-path /tmp/ice_node_test --chain /root/ice-node/frost_testnetRaw.json --scheme Ed25519 --suri 0x605f45a75c60a85b859454dcd43ce124dd1e6f1b1e727f2f5288be3422853580 --password-filename /root/ice-node/password.txt --key-type gran
systemctl restart rpc.service
sleep 8
status=$(systemctl status rpc.service)
echo $status
