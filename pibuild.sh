#!/bin/bash

ARM_TARGET=armv7-unknown-linux-gnueabihf


cross build --release --target $ARM_TARGET && \
    scp ./target/$ARM_TARGET/release/rledsvr pi@ledpi:/home/pi/rledsvr/rledsvr && \
    scp ./rledsvr/deploy/rledsvr.service pi@ledpi:/home/pi/rledsvr/rledsvr.service && \
    scp ./rledsvr/deploy/deploy.sh pi@ledpi:/home/pi/rledsvr/deploy.sh && \
    ssh pi@ledpi.local '/home/pi/rledsvr/deploy.sh'

