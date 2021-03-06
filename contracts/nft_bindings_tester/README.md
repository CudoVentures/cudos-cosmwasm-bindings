# Tester for Cudos-Cosmwasm-Bindings

This is a simple bypassing tester just for testing purpose.

## Running this contract

You will need Rust 1.59+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/bindings_tester.wasm .
ls -l bindings_tester.wasm
sha256sum bindings_tester.wasm
```

Or for a production-ready (compressed) build, run the following from the
repository root:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="cudos_cosmwasm_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

The optimized contracts are generated in the `artifacts/` directory.
