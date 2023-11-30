#!/bin/bash
#
# Node must have been built with benchmarking feature enabled, see ./build_benchmark.sh
#

PALLET=$1

./target/release/logion-node benchmark pallet \
  --chain dev \
  --wasm-execution=compiled \
  --pallet $PALLET \
  --extrinsic "*" \
  --steps 50 \
  --repeat 20 \
  --output runtime/src/weights/$PALLET.rs
