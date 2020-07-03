#!/bin/bash

cross build --release --target armv7-unknown-linux-gnueabihf
scp ./target/armv7-unknown-linux-gnueabihf/release/rledsvr pi@ledpi:/home/pi/rledsvr

