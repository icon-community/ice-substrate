FROM ubuntu:20.04
USER root
WORKDIR /root/ice-node/
RUN apt-get update -y && \
        apt-get install -y wget unzip
RUN wget https://github.com/web3labs/ice-substrate/releases/latest/download/ice-node.zip
RUN unzip ice-node.zip
RUN rm -rvf ice-node.zip
# 30333 for p2p traffic
# 9933 for RPC call
# 9945 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9945 9615 9944
COPY rpc-start.sh /root/ice-node/
RUN chmod 755 /root/ice-node/rpc-start.sh
COPY keystore.txt /root/ice-node/
RUN chmod 755 /root/ice-node/keystore.txt
CMD /bin/bash /root/ice-node/rpc-start.sh
