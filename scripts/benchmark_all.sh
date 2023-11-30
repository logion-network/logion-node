#!/bin/bash
#
# Node must have been built with benchmarking feature enabled, see ./build_benchmark.sh
#

PALLETS=(
  "frame_system" \
  "pallet_balances" \
  "pallet_grandpa" \
  "pallet_multisig" \
  "pallet_recovery" \
  "pallet_sudo" \
  "pallet_timestamp" \
);

for pallet in ${PALLETS[*]}
do
  ./scripts/benchmark_one.sh $pallet
done
