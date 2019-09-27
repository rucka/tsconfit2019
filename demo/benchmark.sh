ROOT=`pwd`

cd $ROOT/ts
echo TypeScript preparation
./yarn.sh
echo TypeScript benchmark
npm run benchmark
echo TypeScript benchmark done

cd $ROOT/rust/native
echo Rust native build
./build.sh
echo Rust native benchmark
./run.sh
echo Rust native benchmark done

cd $ROOT/rust/wasm
echo Rust wasm build
./build.sh
echo Rust wasm benchmark
./run.sh
echo Rust wasm benchmark done

cd $ROOT
