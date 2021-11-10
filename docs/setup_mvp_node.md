# Setup of an additional MVP node

## Generate required keys

Aura and Grandpa keys can be generated with `subkey`. First generate Aura key pair with the following command:

    subkey generate --scheme sr25519

Store the generated output in a safe place then produce the Grandpa key pair

    subkey inspect --scheme ed25519 "$SECRET_PHRASE"

where the SECRET_PHRASE is the one generated for the Aura key pair. Again, store generated output in a safe place.

Finally, generate the node key pair (Logion is a permissioned network):

    subkey generate-node-key

Store the output in a safe place. You'll also need a Base58 encoded form of the Peer ID. You may get it using
[this tool](https://whisperd.tech/bs58-codec/).

Note that the SS58 form of Aura's public key is used to identity the validator and submit extrinsics for configuration.
**It is therefore important to transfer some tokens to it**.

## Start the node

1. Execute the node with at least the following options (note that only relevant options are shown).
```shell
    ./target/release/logion-node \
        ...
        --chain mvp \
        --node-key $PEER_ID \
        --validator
        --offchain-worker always
```

**WARNING:** Do not use the `--rpc-methods Unsafe` option.

2. On the node, create two JSON files for key registration. The content of the files is as follows:
```json
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "author_insertKey",
        "params": ["aura", "$SECRET_PHRASE", "$AURA_PUBLIC_HEX"]
    }
```
for Aura and
```json
    {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "author_insertKey",
        "params": ["gran", "$SECRET_PHRASE", "$GRANDA_PUBLIC_HEX"]
    }
```
for Granpa.

`SECRET_PHRASE` is the same value as above and the public keys come from the output of `subkey` (public keys are **hex formatted**).

Add the keys to the node using curl:

    curl http://localhost:$RPC_PORT -H "Content-Type:application/json;charset=utf-8" -d "@$KEY_FILE"

where `RPC_PORT` is the port configured with `--rpc-port` option and `KEY_FILE` the path to each of the above files.

## Register node's public keys on-chain

If the node was not part of the genesis config, it has to be registered explicitly.

### Authorize node

On another authorized node, go to "Extrinsics" and, as sudo, add the node peer ID to the list of well-known nodes
using call `nodeAuthorization.addWellKnownNode(node, owner)` where `node` is the peer ID (**must be submitted in hex form**) and `owner` the Aura
public key.

The node may have to be restarted before starting to sync blocks.

### Register node as a validator

1. Generate new session keys: Connect to the node of the validator getting added, go to RPC calls and use `author.rotateKeys()` call,
copy generated hex.

2. On another authorized node, add keys for next session: go to "Extrinsics" and, as the new validator, add the keys
using call `session.setKeys(keys, proof)` where `keys` is the hex copied from previous step and `proof` is 0.

3. On another authorized node, activate the validator: go to "Extrinsics" and, as sudo, add the node to the validators set
using call `validatorSet.addValidator(validatorId)` where `validatorId` is the Aura public key (SS58 format).

4. Restart the node.
