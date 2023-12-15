#!/bin/bash
#
# Node must have been built with benchmarking feature enabled, see ./build_benchmark.sh
#

PALLETS=(
  "frame_system" \
  "pallet_balances" \
  "pallet_lo_authority_list" \
  "pallet_logion_loc" \
  "pallet_logion_vote" \
  "pallet_multisig" \
  "pallet_recovery" \
  "pallet_sudo" \
  "pallet_timestamp" \
  "pallet_verified_recovery" \
  "pallet_utility" \
);

for pallet in ${PALLETS[*]}
do
  ./scripts/benchmark_one.sh $pallet
done
