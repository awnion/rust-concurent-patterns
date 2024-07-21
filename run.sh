#!/bin/bash

set -xe

(set +x; echo "GO=========================")
# go run golang/main.go
./golang/main

sleep 1

(set +x; echo "RUST=======================")
cargo build -rq
cargo run -qr
