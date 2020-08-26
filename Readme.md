# SimConnect Bindings for Rust
## Building
*The SimConnect binaries are included within this repository, but they may not be up to date.*

1. Install [CLang](https://clang.llvm.org/get_started.html). More information available at the [Rust Bindgen Documentation](https://rust-lang.github.io/rust-bindgen/requirements.html).
2. run `cargo build`
3. `use simconnectsdk`

### Sample Program
*Note: You must have SimConnect.dll in your current working directory to be able to successfully use SimConnect*
```rust
use simconnectsdk;
use std::time::Duration;
use std::thread::sleep;
use std::mem::transmute_copy;

struct DataStruct {
  lat: f64,
  lon: f64,
  alt: f64,
}

let mut conn = simconnectsdk::SimConnector::new();
conn.connect("Simple Program"); // Intialize connection with SimConnect
conn.add_data_definition(0, "PLANE LATITUDE", "Degrees", simconnectsdk::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX); // Assign a sim variable to a client defined id
conn.add_data_definition(0, "PLANE LONGITUDE", "Degrees", simconnectsdk::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX);
conn.add_data_definition(0, "PLANE ALTITUDE", "Feet", simconnectsdk::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX); //define_id, units, data_type, datum_id
conn.request_data_on_sim_object(0, 0, 0, simconnectsdk::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME); //request_id, define_id, object_id (user), period - tells simconnect to send data for the defined id and on the user aircraft

loop {
  match conn.get_next_message() {
    Ok(simconnectsdk::DispatchResult::SimobjectData(data)) => {
      unsafe {
        match (*data).dwDefineID {
          0 => {
            let sim_data: DataStruct = transmute_copy(&(*data).dwData);
            println!("{:?} {:?} {:?}", sim_data.lat, sim_data.lon, sim_data.alt);
          },
          _ => ()
        }
      }
    },
    _ => ()
  }
  
  sleep(Duration::from_millis(16)); // Will use up lots of CPU if this is not included, as get_next_message() is non-blocking
}
```
### Remarks
I have not tested every single function from the api. If you find an error, feel free to make an issue or a pull request.
