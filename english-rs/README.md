Build: `cargo build --target wasm32-unknown-unknown`
Package: `wasm-tools component new ./target/wasm32-unknown-unknown/debug/english_rs.wasm -o english_rs.wasm`
(`cargo install wasm-tools` first!)

Then copy the `english_rs.wasm` file to the `plugins` folder of `i18n-greeter`.
