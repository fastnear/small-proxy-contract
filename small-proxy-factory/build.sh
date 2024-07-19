#!/bin/bash
set -e

pushd $(dirname ${BASH_SOURCE[0]})
mkdir -p res

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/small_proxy_factory.wasm ./res/

ls -lsa res/small_proxy_factory.wasm
