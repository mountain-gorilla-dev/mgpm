FROM rust:1.75.0-slim-bullseye

WORKDIR /home/user/.mgpm

USER user

RUN apt update && \
    apt install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        git

RUN rustup component add rustfmt && \
    rustup component add clippy
