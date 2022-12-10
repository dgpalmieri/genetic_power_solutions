#! /bin/env bash

while true; do
    RUST_BACKTRACE=1 cargo run | tee -a cargo_output_$(date -I).log
done;
