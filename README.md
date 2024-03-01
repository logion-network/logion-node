# Logion node

This project exposes the Logion network's nodes.

## Chain specification

Logion nodes implementation is based on
[Substrate Node Template v3.0](https://github.com/substrate-developer-hub/substrate-node-template/releases/tag/v3.0.0%2B1).

The logion chain exposes the following features:
- [Accounts and balances](https://substrate.dev/rustdocs/v3.0.0/pallet_balances/index.html)
- [Multi-signature](https://substrate.dev/rustdocs/v3.0.0/pallet_multisig/index.html)
- [Proxying](https://substrate.dev/rustdocs/v3.0.0/pallet_proxy/index.html)
- [Social recovery](https://substrate.dev/rustdocs/v3.0.0/pallet_recovery/index.html)

The logion network is permissioned. The list of "well-known" nodes (i.e. nodes that are authorized to
validate blocks) is managed by the root user (Alice for the moment). The permissioned network was configured by
following [this tutorial](https://substrate.dev/docs/en/tutorials/build-permission-network/).

New validators have to generate their node key and communicate the peer ID and owner account to a root user in order
to be added to the
list of well known nodes and start validating blocks. Node keys may be generated
using `subkey`, see [here](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#generating-node-keys).

Finally, here are the general parameters of Logion nodes' Runtime:
- Block time: 6s
- Hash algorithm: Blake2
- Hash size: 256-bits
- Block number: 32 bits
- Account index depth: 32 bits
- Account balance depth: 128 bits
- Transaction chain index depth: 32 bits
- Block authoring: [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura)
- Block finalization: [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa)
- Database: [RocksDb](https://rocksdb.org/)

## Getting Started

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Run for development

Below command will run the node in development mode with a temporary storage.

```sh
./target/release/logion-node --dev
```

## Logion Components

* The [Node](https://github.com/logion-network/logion-node) is the implementation of the chain.
* The [Typescript backend](https://github.com/logion-network/logion-backend-ts) stores data which cannot be exposed publicly, or which wait legal officer's approval.
* The [Wallet](https://github.com/logion-network/logion-wallet) is the user application.

## Chainspec

The following commands can be used to rebuild testnet chainspec files in `res` folder (`$ENV` is one of dev or test):

```
./target/release/logion-node build-spec --disable-default-bootnode --chain test > ./res/$ENV-plain.json
./target/release/logion-node build-spec --chain ./res/$ENV-plain.json --raw --disable-default-bootnode > ./res/$ENV-raw.json
```

## Try Runtime

`try-runtime` tool enables the testing of a new runtime against real data.

### Test a runtime upgrade

Generally, what's tested here is one or several storage migrations activated by the new runtime or any Polkadot upgrade.

If not yet done, the [Substrate Try Runtime CLI](https://github.com/paritytech/try-runtime-cli) must be installed:

```sh
cargo install --git https://github.com/paritytech/try-runtime-cli --locked
```

If not yet done, the runtime has to be built with the `try-runtime` feature:

```sh
cargo build --release --features=try-runtime
```

It can then be tested by executing the following command:

```sh
try-runtime --runtime target/release/wbuild/logion-node-runtime/logion_node_runtime.compact.compressed.wasm on-runtime-upgrade live --uri wss://rpc01.logion.network:443
```

This will:
- connect to RPC node
- download current state
- execute the upgrade
- run pallets' `post_upgrade` hook

## Benchmark

In order to run benchmarks, the runtime has to be built with the right features enabled:

```
./scripts/build_benchmark.sh
```

All required benchmarks can be run by executing the following command:

```
./scripts/benchmark_all.sh
```

Pallet benchmarks can be run one at a time with the following command:

```
./scripts/benchmark_one.sh $PALLET
```

with $PALLET being a pallet name listed in [`runtime/src/weights`](`./runtime/src/weights`) (the file name without extesion `.rs`).

### Benchmarking a new pallet

1. `./runtime/Cargo.toml`: in section `[features]`, add pallet to `runtime-benchmarks` array.
2. `./runtime/src/lib.rs`:
  - If not already done, configure pallet with default `WeightInfo` (`type WeightInfo = ();`).
  - Add pallet to `define_benchmarks!` arguments
3. Re-build runtime for benchmarks (see above); a failure here may mean that the pallet does not have any benchmark or benchmark code is broken.
4. Run `./scripts/list_benchmarks.sh` and make sure that added pallet appears. If it is not the case, `cargo clean` then restart from step 3.
5. Run benchmark for the new pallet (see above); a failure here (no module file produced) may mean that benchmark code is broken.
   If the bencharmks are not
6. `./runtime/src/weights.rs`: Add newly generated module.
7. `./runtime/src/lib.rs`: Pass generated `WeightInfo` to pallet (`type WeightInfo = weights::PALLET_NAME::WeightInfo<Runtime>;`).
8. Run `cargo check`; a failure here may mean that pallet benchmarks were not updated following a pallet API change.
9. `./scripts/benchmark_all.sh`: add pallet to `PALLETS` array.

## Build for release

- Build using [`srtool`](https://docs.substrate.io/reference/command-line-tools/srtool/)
  - `cp rust-toolchain.toml runtime/`
  - `srtool build --root --package logion-node-runtime --runtime-dir runtime`
  - `rm runtime/rust-toolchain.toml`

The toolchain file must be copied to prevent an issue about missing `rust-src` during build. This step (and clean-up)
may be removed once it is added to srtool's [Docker image](https://github.com/paritytech/srtool/blob/master/Dockerfile).
