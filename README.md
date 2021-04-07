# Logion node prototype

A prototype based on the
[Substrate Node Template v3.0](https://github.com/substrate-developer-hub/substrate-node-template/releases/tag/v3.0.0).

## Chain specifications

Logion nodes expose the following features:
- [Accounts and balances](https://substrate.dev/rustdocs/v3.0.0/pallet_balances/index.html)
- [Multi-signature of transactions](https://substrate.dev/rustdocs/v3.0.0/pallet_multisig/index.html)

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

Follow these steps to get started :hammer_and_wrench:

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a
```

The node key is linked to the peer ID used in the Genesis config.

Logion nodes are not on the same local network,so we don't need mDNS and should use --no-mdns to disable it.
Reachable nodes should be provded with --reserved-nodes flag.

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
