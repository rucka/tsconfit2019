ROOT=`pwd`
CARGO_REGISTRY=$ROOT/workspace/registry
RUST_NATIVE_TARGET=$ROOT/workspace/native-target
RUST_WASM_TARGET=$ROOT/workspace/wasm-target
TS_NODE_MODULES=$ROOT/workspace/ts-node-modules

mkdir -p $ROOT/workspace/registry
docker run -t -i --rm \
  -v="$ROOT:/demo" \
  -v="$CARGO_REGISTRY:/usr/local/cargo/registry" \
  -v="$RUST_NATIVE_TARGET:/demo/rust/native/target" \
  -v="$RUST_WASM_TARGET:/demo/rust/wasm/target" \
  -w /demo tsconfit2019-demo:latest bash
