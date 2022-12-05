#! /bin/env bash

while true; do
    cargo run | tee -a cargo_output_${date -I}.log
done;
