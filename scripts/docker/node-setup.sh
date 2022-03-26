#!/bin/bash
cd rpc/
echo "SecureKey" > keystore.txt
docker build -t rpc-image .
docker network create --driver bridge --subnet 172.19.0.0/24 --gateway 172.19.0.1 --ip-range 172.19.0.0/24 docker-rpc-network
docker run -d --name rpc-node -p 30333:30333 -p 9944:9944 -p 9945:9945 -p 9615:9615 -p 9933:9933 --network docker-rpc-network --ip 172.19.0.10 rpc-image
sleep 3
cd ../validator/
echo "SecureKey" > keystore.txt
echo IDENTITY=$(docker logs rpc-node 2>&1 | head -n14 | awk -F 'Local node identity is: ' '{print $2}') > localid
docker build -t validator-image .
docker run -d --name validator-node -p 30334:30333 --env-file ./localid --network docker-rpc-network --ip 172.19.0.11 validator-image
