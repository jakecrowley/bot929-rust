FROM ubuntu:18.04

RUN apt update && apt install -y curl build-essential

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.72.1 -y

