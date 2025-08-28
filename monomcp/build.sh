#!/bin/bash

export RUST_MIN_STACK=16777216

cargo build 2>&1 | tee /data/data/com.termux/files/home/storage/github/mcp/monomcp/log.txt
