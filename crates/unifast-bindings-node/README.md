# unifast-bindings-node

[![Crates.io](https://img.shields.io/crates/v/unifast-bindings-node)](https://crates.io/crates/unifast-bindings-node)

[napi-rs](https://napi.rs/) Node.js binding for [unifast-core](https://crates.io/crates/unifast-core).

This crate compiles to a native Node.js addon (`.node` file) that exposes unifast's Rust compiler to JavaScript via N-API.

> **Note:** Most users should install the npm package [`@unifast/node`](https://www.npmjs.com/package/@unifast/node) instead of using this crate directly.

## How It Works

This crate wraps `unifast-core`'s compilation API using `napi-rs`, producing a `cdylib` that Node.js can load as a native addon. The npm package `@unifast/node` bundles prebuilt binaries for multiple platforms (Linux x64/arm64, macOS x64/arm64, Windows x64).

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
