FROM rust:1.75.0-slim-buster

WORKDIR /root/app

RUN apt update && \
    apt install -y \
        build-essential \
        pkg-config \
        libssl-dev
