#!/bin/bash
cargo b --release
sudo setcap cap_net_admin=eip target/release/tcp
cargo r --release 
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
trap cleanup INT TERM
