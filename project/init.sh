#!/bin/bash

export TERM=xterm-256color

service mysql start
mysql < /project/mysql/setup.sql

#cargo test # ensure dependencies are set up

tail -F -n0 /etc/hosts
