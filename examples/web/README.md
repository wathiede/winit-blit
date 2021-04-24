# Example for targeting `wasm32-unknown-unknown`

This is an example that has the same functionality as
[../blit.rs](../blit.rs), but runs in the webbrowser.  Some additional code is
required, and the crate should be a library with config in `Cargo.toml` like:

```toml
[lib]
crate-type = ["cdylib"]
```

## Setup
Building and running this example requires three things:

1. You have the `wasm32-unknown-unknown` target installed for your rust
   toolchain.
1. You have `wasm-pack`
   [installed](https://rustwasm.github.io/wasm-pack/installer/)
1. You have something that can serve static HTTP content (`index.html` and
   `pkgs/*`).

## Build and Run Example

1. Build with `wasm-pack build --debug --target web`
1. Run a static webserver i.e. `miniserve -p 8080 .` or `python -m http.server 8080`
1. Visit [http://localhost:8080](http://localhost:8080) and you should see
   nice grey gradient filled boxes.

