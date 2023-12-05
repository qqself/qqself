# core-bindings-wasm

Contains WASM bindings for the qqself core. We are using [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to generate bindings. Not everything is exposed from the core, but only the main features to support [client-web](../client-web/).

## Development

Few hard learned rules to follow when writing binding functions:

- panics/unreachable/todo should not be used as it breaks WebAssembly context and bridge stops. Return `Result<T, String>` instead
- Never pass structs by value as then WebAssembly will nullify this object on JS side
- Use `crate::util::{log, error}` for debugging
- Use similar rules as `uniffi` and avoid `&mut self` and use interior mutability instead
- Careful with recursion - if Rust calls passed JS function (e.g. callback) which in turn calls Rust again it may create a
  situation where struct is borrowed twice. To break recursion use `setTimeout(logic, 0)` on JS side
- All binding functions should be shallow and without any logic. If function is longer than one line consider to add it to the `core` instead
- `core` with enabled `wasm` feature can use `#[wasm_bindgen]`. Sometimes there no need to create a WASM friendly struct wrapper and we can return a `core` struct directly
