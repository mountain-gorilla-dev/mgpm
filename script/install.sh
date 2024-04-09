#!/bin/bash

# linux

set -e

OS=$(uname -s)
VERSION="0.1.1"
ARCH=$(uname -m)

if [ ${OS} = "Darwin" ]; then
  echo "Mac OS です"
  INSTALL_PATH=/usr/local/bin/mgpm
  if [ ${ARCH} = "arm64" ]; then
    ARCH="aarch64"
  fi
  URL=https://github.com/mountain-gorilla-dev/mgpm/releases/download/$VERSION/mgpm-darwin-$ARCH
  wget -O $INSTALL_PATH $URL
  chmod +x $INSTALL_PATH
  exit
elif [ ${OS} = "Linux" ]; then
  echo "Linuxです"
  exit
else
  echo "対応していないOSです"
  exit
fi
