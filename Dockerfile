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

CMD ./logion-node \
--alice \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--validator \
--chain=local \
--base-path $DATA_DIRECTORY \
--port $P2P_PORT \
--ws-port $WS_PORT \
--rpc-port $RPC_PORT \
--rpc-cors all \
--rpc-methods Unsafe \
--ws-external

EXPOSE ${WS_PORT}
EXPOSE ${P2P_PORT}
EXPOSE ${RPC_PORT}
