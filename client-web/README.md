# client-web

qqself web client. Built using `TypeScript` and `lit-element`. Using `core` via WebAssembly bindings [core-binding-wasm](../core-bindings-wasm) wrapper.

Tests are running using `Jest` in `NodeJS` which supports WebAssembly just fine.

## Development

Using `vite`, hot reload is supported for both TypeScript and Rust. Run `yarn start` to serve the client. Whenever Rust part has changed run `yarn build` in another terminal to rebuild the wasm.

Minimum `NodeJS` version is 18 as we are using `fetch` API in tests
