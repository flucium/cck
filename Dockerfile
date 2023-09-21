FROM ubuntu:22.04

RUN apt update && apt upgrade -y && \
apt install curl git build-essential -y && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
rustup toolchain install nightly && \
rustup default nightly && \
mkdir /repos

ENV PATH="/root/.cargo/bin:$PATH"

CMD /bin/bash