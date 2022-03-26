FROM ubuntu:20.04
ENV IDENTITY default
USER root
WORKDIR /root/ice-node/
RUN apt-get update -y && \
        apt-get install -y wget unzip
RUN wget https://github.com/web3labs/ice-substrate/releases/latest/download/ice-node.zip
RUN unzip ice-node.zip
RUN rm -rvf ice-node.zip
# 30333 for p2p traffic
EXPOSE 30333
COPY validator-start.sh /root/ice-node/
RUN chmod 755 /root/ice-node/validator-start.sh
COPY keystore.txt /root/ice-node/
RUN chmod 755 /root/ice-node/keystore.txt
CMD /bin/bash /root/ice-node/validator-start.sh $IDENTITY
