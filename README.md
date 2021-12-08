# Web Games

## Development

### Install dependenices

- Rust
- Deps
  ```bash
  rustup target add wasm32-unknown-unknown
  cargo install trunk wasm-bindgen-cli cargo-watch
  ```

### Run and recompile on save

```bash
cargo watch -q -w server -x "run --bin server"
trunk watch .\app\index.html -w app
```
