#!/bin/sh

printf -v day "day%02d" $2
cargo run --manifest-path=$1/$day/Cargo.toml --release
