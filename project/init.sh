#!/bin/bash

export TERM=xterm-256color
export RUST_LOG=info

service mysql start
mysql < /project/mysql/setup.sql
mysql < /project/mysql/20170225.sql
mysql < /project/mysql/20170303.sql
mysql < /project/mysql/20170325.sql

#cargo test # ensure dependencies are set up

#cargo run
tail -F -n0 /etc/hosts
