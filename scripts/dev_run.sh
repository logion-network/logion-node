#!/bin/bash


# Node key of Alice, needed because network is permissioned, see Development node genesis for accepted nodes
NODE_KEY=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a

./target/release/logion-node --dev --tmp --node-key=$NODE_KEY
