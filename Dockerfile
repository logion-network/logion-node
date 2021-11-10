# Build node
FROM paritytech/ci-linux:production AS build
WORKDIR /tmp/logion-node
COPY . .
RUN cargo build --release

# Backend image
FROM ubuntu:hirsute
WORKDIR /usr/share/logion-node
COPY --from=build /tmp/logion-node/target/release/logion-node logion-node

ENV P2P_PORT=30333
ENV WS_PORT=9944
ENV RPC_PORT=9933
ENV DATA_DIRECTORY=./data
ENV RUST_LOG=info
ENV NODE_KEY=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a
ENV CHAIN_SPEC=local
ENV CUSTOM_OPTIONS="--alice --ws-external --rpc-cors all"

CMD ./logion-node \
$CUSTOM_OPTIONS \
--node-key $NODE_KEY \
--chain $CHAIN_SPEC \
--base-path $DATA_DIRECTORY \
--port $P2P_PORT \
--ws-port $WS_PORT \
--rpc-port $RPC_PORT

EXPOSE ${WS_PORT}
EXPOSE ${P2P_PORT}
EXPOSE ${RPC_PORT}
