#!/bin/bash

sudo systemctl stop rledsvr

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
RELDIR="$SCRIPT_DIR/../../"
PROJECT_ROOT_DIR=$(readlink -f $RELDIR)
BIN_DIR="$PROJECT_ROOT_DIR/target/release"

if [ -d /var/rpiled ]; then
    sudo rm -rf /var/rpiled
fi

sudo mkdir -p /var/rpiled

sudo cp "$BIN_DIR/rledsvr" /var/rpiled/rledsvr
sudo cp "$SCRIPT_DIR/rledsvr.service" /var/rpiled/rledsvr.service

sudo systemctl enable /var/rpiled/rledsvr.service
sudo systemctl start rledsvr
sudo systemctl status rledsvr
