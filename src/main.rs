use simconnect;
use std::time::Duration;
use std::thread::sleep;
use std::mem::transmute_copy;

struct DataStruct {
    lat: f64,
    lon: f64,
    alt: f64,
}

pub fn main() {
    let mut conn = simconnect::SimConnector::new();
    conn.connect("Simple Program"); // Intialize connection with SimConnect
    conn.add_data_definition(0, "PLANE LATITUDE", "Degrees", simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX); // Assign a sim variable to a client defined id
    conn.add_data_definition(0, "PLANE LONGITUDE", "Degrees", simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX);
    conn.add_data_definition(0, "PLANE ALTITUDE", "Feet", simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX); //define_id, units, data_type, datum_id
    conn.request_data_on_sim_object(0, 0, 0, simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME, 0, 0, 0, 0); //request_id, define_id, object_id (user), period, falgs, origin, interval, limit - tells simconnect to send data for the defined id and on the user aircraft

    loop {
        match conn.get_next_message() {
            Ok(simconnect::DispatchResult::SimobjectData(data)) => {
                unsafe {
                    match data.dwDefineID {
                        0 => {
                            let sim_data: DataStruct = transmute_copy(&data.dwData);
                            println!("{:?} {:?} {:?}", sim_data.lat, sim_data.lon, sim_data.alt);
                        },
                        _ => ()
                    }
                }
            },
            Err(e) => {
                eprintln!("Error was received: {}", e);
            },
            _ => ()
        }

        sleep(Duration::from_millis(16)); // Will use up lots of CPU if this is not included, as get_next_message() is non-blocking
    }
}
