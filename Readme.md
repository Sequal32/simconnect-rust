# SimConnect Bindings for Rust
## Usage
Add this to your `Cargo.toml`
```toml
[dependencies]
simconnect = "0.2.0"
```
*This crate is in its early stages and API breaking changes can be pushed at any moment. It's recommended to use the exact version "0.X.X"*

## Example
```
cargo run --example connection
```

*You must have SimConnect.dll in the same directory as the compiled exe for it to run (eg. in )*

## Building
*The SimConnect.dll is included in this repository, but might not be up to date*

1. Install [CLang](https://clang.llvm.org/get_started.html). More information available at the [Rust Bindgen Documentation](https://rust-lang.github.io/rust-bindgen/requirements.html).
2. run `cargo build`
3. `use simconnect`

### Remarks
I have not tested every single function from the api. If you find an error, feel free to make an issue or a pull request.
