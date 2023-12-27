use std::thread::sleep;
use std::time::Duration;

use simconnect::DispatchResult;

struct DataStruct {
    lat: f64,
    lon: f64,
    alt: f64,
}
fn main() {
    let mut conn = simconnect::SimConnector::new();
    conn.connect("Simple Program"); // Intialize connection with SimConnect
    conn.add_data_definition(
        0,
        "PLANE LATITUDE",
        "Degrees",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
        u32::MAX,
        0.0,
    ); // Assign a sim variable to a client defined id
    conn.add_data_definition(
        0,
        "PLANE LONGITUDE",
        "Degrees",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
        u32::MAX,
        0.0,
    );
    conn.add_data_definition(
        0,
        "PLANE ALTITUDE",
        "Feet",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
        u32::MAX,
        1.0,
    ); //define_id, units, data_type, datum_id, epsilon (update threshold)
    conn.request_data_on_sim_object(
        0,
        0,
        0,
        simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
        0,
        0,
        0,
        0,
    ); //request_id, define_id, object_id (user), period, falgs, origin, interval, limit - tells simconnect to send data for the defined id and on the user aircraft

    loop {
        match conn.get_next_message() {
            Ok(DispatchResult::SimobjectData(data)) => unsafe {
                if data.dwDefineID == 0 {
                    let sim_data_ptr = std::ptr::addr_of!(data.dwData) as *const DataStruct;
                    let sim_data_value = std::ptr::read_unaligned(sim_data_ptr);
                    println!(
                        "{:?} {:?} {:?}",
                        sim_data_value.lat, sim_data_value.lon, sim_data_value.alt
                    );
                }
            },
            Ok(DispatchResult::Open(_)) => {
                println!("Connected to simulator.");
            }
            Ok(DispatchResult::Quit(_)) => {
                println!("Disconnected from simulator.");
            }
            _ => (),
        }

        sleep(Duration::from_millis(16)); // Will use up lots of CPU if this is not included, as get_next_message() is non-blocking
    }
}
