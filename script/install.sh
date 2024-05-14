#!/bin/bash

# mac&linux

set -e

OS=$(uname -s)
ARCH=$(uname -m)

INSTALL_PATH=/usr/local/bin/mgpm
if [ ${ARCH} = "arm64" ]; then
  ARCH="aarch64"
fi
URL=https://github.com/mountain-gorilla-dev/mgpm/releases/latest/download/mgpm-$OS-$ARCH
wget -O $INSTALL_PATH $URL
chmod +x $INSTALL_PATH
exit
