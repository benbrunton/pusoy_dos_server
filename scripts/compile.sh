#!/bin/bash

rust-musl-builder() {
    docker run --rm -it \
        -v "$(pwd)/project":/home/rust/src \
        -v cargo-registry:/home/rust/.cargo/registry \
        ekidd/rust-musl-builder "$@"
}

chmod 777 "$(pwd)/project/target"
rust-musl-builder cargo build # --release
