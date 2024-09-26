#!/bin/bash

BINARY_PATH="target/release/tcp-no-reason"
TUN_INTERFACE="tun0"
TUN_IP="192.168.0.1/24"

cargo b --release
sudo setcap cap_net_admin=eip $BINARY_PATH
./$BINARY_PATH &

pid=$!

echo "we gucci"

sudo ip addr add $TUN_IP dev $TUN_INTERFACE
sudo ip link set up dev $TUN_INTERFACE

trap "kill $pid" INT TERM
wait $pid 


