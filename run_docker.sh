#!/bin/bash

set -xe

docker build -t rust-concurent-experiment .

(set +x; echo "GO=========================")
docker run rust-concurent-experiment /go_main

sleep 1

(set +x; echo "RUST=======================")
docker run rust-concurent-experiment /rust_main
