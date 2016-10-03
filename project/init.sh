#!/bin/bash

service mysql start
mysql < /project/mysql/setup.sql

tail -F -n0 /etc/hosts
