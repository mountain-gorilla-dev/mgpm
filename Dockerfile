FROM rust:1.76.0-slim-bookworm

ARG USERNAME=user
ARG GROUPNAME=${USERNAME}
ARG UID=1000
ARG GID=1000
ARG PASSWORD=user
ARG WORKDIR=/home/${USERNAME}/.mgpm

RUN apt update && \
    apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    sudo

RUN groupadd -g $GID ${GROUPNAME} && \
    useradd -m -s /bin/bash -u $UID -g $GID -G sudo ${USERNAME} && \
    echo ${USERNAME}:$PASSWORD | chpasswd && \
    echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

RUN mkdir -p ${WORKDIR}/target && \
    chown -R ${USERNAME}:${GROUPNAME} ${WORKDIR}/target

USER ${USERNAME}
WORKDIR ${WORKDIR}

RUN cargo install cargo-watch

RUN rustup component add rustfmt && \
    rustup component add clippy
