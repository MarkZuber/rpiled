#!/bin/bash

ARM_TARGET=armv7-unknown-linux-gnueabihf


cross build --release --target $ARM_TARGET && \
    ssh pi@ledpi.local 'sudo systemctl stop rledsvr' && \
    scp ./target/$ARM_TARGET/release/rledsvr pi@ledpi:/home/pi/rledsvr/rledsvr && \
    rsync -zvhr ./rledsvr/deploy/ pi@ledpi:/home/pi/rledsvr/ && \
    rsync -zvhr ./content/ pi@ledpi:/home/pi/rledsvr/content/ && \
    ssh pi@ledpi.local '/home/pi/rledsvr/deploy.sh'

