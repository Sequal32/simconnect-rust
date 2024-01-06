![crates.io](https://img.shields.io/crates/v/simconnect)
# SimConnect Bindings for Rust
## Using
Add this to your `Cargo.toml`
```toml
[dependencies]
simconnect = "0.3.0"
```

## Building
*The SimConnect binaries are included within this repository, but they may not be up-to-date.*

1. Install [CLang](https://clang.llvm.org/get_started.html). More information available at the [Rust Bindgen Documentation](https://rust-lang.github.io/rust-bindgen/requirements.html).
2. run `cargo build`
3. Add `use simconnect` at the top of your file

## Example
Read float position data

```
cargo run --example aircraft_updates
```

Requests tagged data with thresholds from SimConnect and reads floats/strings
```
cargo run --example aircraft_updates_on_change
```

*You must have SimConnect.dll in the same directory as the compiled exe for it to run (e.g. in )*

## Building
*The SimConnect.dll is included in this repository, but might not be up-to-date*

### Remarks
I have not tested every single function from the api. If you find an error, feel free to make an issue or a pull request.
