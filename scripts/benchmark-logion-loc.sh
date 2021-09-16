#!/bin/bash
#
# Node must have been built with benchmarking feature enabled, see ./build_benchmark.sh
#

./target/release/logion-node benchmark \
    --execution wasm \
    --wasm-execution compiled \
    --pallet pallet_logion_loc \
    --extrinsic '*' \
    --steps 20 \
    --repeat 10 \
    --output ./pallets/logion_loc/src/weights.rs \
    --template ./scripts/weights-template.hbs \
