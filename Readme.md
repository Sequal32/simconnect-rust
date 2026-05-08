![crates.io](https://img.shields.io/crates/v/simconnect)

# SimConnect Bindings for Rust

Send and receive information through SimConnect, for Microsoft Flight Simulator (2020).

Documentation can be found at: [MSFS 2020 SDK](https://docs.flightsimulator.com/html/Programming_Tools/SimConnect/SimConnect_SDK.htm)


## Requirements

- [Rust](https://rust-lang.org/learn/get-started)
- [LLVM/Clang](https://clang.llvm.org/get_started.html)

See the [Rust Bindgen Documentation](https://rust-lang.github.io/rust-bindgen/introduction.html)


## Using

Add this to your `Cargo.toml`

```toml
[dependencies]
simconnect = "0.4"
```

_You must have SimConnect.dll in the same directory as your executable._


## Building

_The SimConnect binaries are included within this repository, but they may not be up-to-date._

1. Run `cargo build`
2. Add import `use simconnect` at the top of your file


## Examples

Read float position data

```
cargo run --example aircraft_updates
```

Requests tagged data with thresholds from SimConnect, and reads floats/strings

```
cargo run --example aircraft_updates_on_change
```


## Remarks

I have not tested every function of the API. If you find an error, feel free to make an issue or pull request.
