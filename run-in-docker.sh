#!/bin/bash

set -exo pipefail

RUST_IMAGE=rust:latest

# Run the docker command - using the same build directory as the host machine
docker run \
        --rm \
        -v`pwd`:`pwd` \
        --name docker-rustc \
        --security-opt seccomp=unconfined \
        -e CARGO_HOME=`pwd`/target/pkg_cache \
        -u $(id -u) \
        -w `pwd` \
        $RUST_IMAGE "$@"
