# unifast-bindings-wasm

[![Crates.io](https://img.shields.io/crates/v/unifast-bindings-wasm)](https://crates.io/crates/unifast-bindings-wasm)

[wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) WebAssembly binding for [unifast-core](https://crates.io/crates/unifast-core).

This crate compiles to a WASM module, allowing unifast's Rust Markdown/MDX compiler to run in the browser.

## How It Works

Uses `wasm-bindgen` to expose `unifast-core`'s compilation API to JavaScript running in WebAssembly-capable environments (browsers, Deno, Cloudflare Workers, etc.).

## License

[MIT](https://github.com/kenzo-pj/unifast/blob/main/LICENSE)
