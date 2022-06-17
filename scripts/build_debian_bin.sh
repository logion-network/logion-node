#!/bin/bash

BUILDER_IMAGE=logionnetwork/logion-node-debian-builder:latest

docker build . -f ./Dockerfile.build-debian11 -t $BUILDER_IMAGE
docker run --rm -it -v $(pwd)/debian-bin:/target/logion-node $BUILDER_IMAGE