# Setup of an additional production node

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


## Authorize new node

On an already authorized node, go to "Extrinsics" and, as sudo, add the node peer ID to the list of well-known nodes
using call `nodeAuthorization.addWellKnownNode(node, owner)` where `node` is the peer ID (**must be submitted in hex form**) and `owner` the Aura
public key.


## Start the node

Execute the node with at least the following options (note that only relevant options are shown).
```shell
    ./target/release/logion-node \
        ...
        --chain mvp \
        --node-key $PEER_ID \
        --validator
        --offchain-worker always
```

**WARNING:** Do not use the `--rpc-methods Unsafe` option.

Before going further, let it synchronize.


## Register keys

On the node's host, run the following commands:

`curl http://localhost:$RPC_PORT -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc": "2.0", "id": 1, "method": "author_insertKey", "params": ["aura", "$SECRET_PHRASE", "$AURA_PUBLIC_HEX"]}'`

for Aura and

`curl http://localhost:$RPC_PORT -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc": "2.0","id": 1,"method": "author_insertKey","params": ["gran", "$SECRET_PHRASE", "$GRANDA_PUBLIC_HEX"]}'`

for Granpa.

`SECRET_PHRASE` is the same value as above and the public keys come from the output of `subkey` (public keys are **hex formatted**).
`RPC_PORT` is the port configured with `--rpc-port` option (9933 by default).

**WARNING: You may have to remove the above entries from your bash history in order to prevent the leak of the secret phrase.**


## Register node as a validator

1. Generate new session keys: on the new node, run the following command and copy 
generated hex.

`curl http://localhost:$RPC_PORT -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc": "2.0", "id": 1, "method": "author_rotateKeys", "params": []}'`

2. On an already authorized node, add keys for next session: go to "Extrinsics" and, as the new validator, add the keys
using call `session.setKeys(keys, proof)` where `keys` is the hex copied from previous step and `proof` is 0.

3. On another authorized node, activate the validator: go to "Extrinsics" and, as sudo, add the node to the validators set
using call `validatorSet.addValidator(validatorId)` where `validatorId` is the Aura public key (SS58 format).


## Check the logs and Polkadot JS apps

You should now see the node part of the validators set and authoring blocks.
