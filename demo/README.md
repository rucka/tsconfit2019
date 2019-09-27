This is the code of the benchmarks that support the talk.

There are three versions of the benchmarks, in different directories:
* `ts` (for Typescript)
* `rust/native` (for Rust, compiling to native code)
* `rust/wasm` (for Rust, compiling to wasm)

The two Rust versions share the implementation in a common crate (`rust/runner`).

If you have an environment with recent releases of:

* nodejs
* rust
* wasm-pack

you can run the `benchmark.sh` script from this directory and it will build and run the benchmarks.

The first time there will be longish builds, after that the experience will be better (build results are not thrown away).

If you do not have Rust or Node the `Dockerfile` describes a docker image capable of running the benchmarks.
Run the `build.sh` script to build and tag the image.

Then running the `shell.sh` script will then open a shell inside a container started from that image.
From there the `benchmark.sh` script will work.

The `shell.sh` script will set up lots of volumes so that the containerized environment will not overwrite target files in the source tree, and will at the same time keep state for incremental builds. This way the `benchmark.sh` script can be run either from inside the container and directly on the host, and the two environments will not influence each other.

If you want to throw away this state remove the `workspace` directory (note that its contents, being generated from inside the container, will be mostly owned by `root`).
