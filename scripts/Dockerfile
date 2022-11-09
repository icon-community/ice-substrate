# This is the build stage for Snow. Here we create the binary in a temporary image.
FROM phusion/baseimage:focal-1.0.0 as builder
WORKDIR /snow
COPY . /snow
RUN apt-get update && \
        apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
        apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev build-essential

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH="$PATH:$HOME/.cargo/bin" && \
        rustup default nightly-2022-01-16 && \
        rustup target add wasm32-unknown-unknown --toolchain nightly-2022-01-16 && \
        git clone https://github.com/web3labs/ice-substrate.git && \
        cd ice-substrate/ && \
        cargo build --release
# This is the 2nd stage: a very small image where we copy the Snow binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Snow Docker image" \
        maintainer="Web3 Labs" \
        io.parity.image.type="builder"

COPY --from=builder /snow/ice-substrate/target/release/ice-node /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /snow snow && \
        mkdir -p /data /snow/.local/share && \
        chown -R snow:snow /data && \
        ln -s /data /snow/.local/share/snow && \
# unclutter and minimize the attack surface
        rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
        /usr/local/bin/ice-node --version

USER snow

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/ice-node"]
