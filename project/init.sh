#!/bin/bash

export TERM=xterm-256color
export RUST_LOG=info

service mysql start
mysql < /project/mysql/setup.sql

#cargo test # ensure dependencies are set up

#tail -F -n0 /etc/hosts
cargo run
