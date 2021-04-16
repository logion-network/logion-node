# Logion node prototype

This project exposes the prototype of Logion network's nodes.

## Chain specification

Logion nodes implementation is based on
[Substrate Node Template v3.0](https://github.com/substrate-developer-hub/substrate-node-template/releases/tag/v3.0.0).

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

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Run for development

Below command will run the node in development mode with a temporary storage.

```sh
./scripts/dev_run.sh
```

See script for details.

## Logion UI

See [here](https://github.com/logion-network/logion-frontend-prototype).
