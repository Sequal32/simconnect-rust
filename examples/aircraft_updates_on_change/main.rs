use simconnect;
use simconnect::DWORD;
use std::time::Duration;
use std::thread::sleep;

// To allign the memory we have to set a fixed max size to the returned variables from the game
const MAX_RETURNED_ITEMS: usize = 255;

#[repr(C, packed)]
struct KeyValuePairFloat {
    id: DWORD,
    value: f64
}
struct DataFloatStruct {
    data: [KeyValuePairFloat; MAX_RETURNED_ITEMS]
}
#[repr(C, packed)]
struct KeyValuePairString {
    id: DWORD,
    // Strings get returned as max 255 bytes
    value: [u8; 255],
}

struct DataStringStruct {
    data: [KeyValuePairString; MAX_RETURNED_ITEMS]
}

fn main() {
    let mut conn = simconnect::SimConnector::new();
    conn.connect("Program that returns data on changes"); // Intialize connection with SimConnect

    // Here we define all our variable that get returned as floats
    // (including integers, the memory allignment will handle the)
    conn.add_data_definition(0, "PLANE LATITUDE", "Degrees", simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, 1, 1.0); // Assign a sim variable to a client defined id
    conn.add_data_definition(0, "PLANE LONGITUDE", "Degrees", simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, 2, 1.0);
    conn.add_data_definition(0, "PLANE ALTITUDE", "Feet", simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, 3, 1.0); //define_id, units, data_type, datum_id, epsilon (update threshold)

    // Here we define all our variabes that get returned as Strings
    // Notice how the define_id differs from the float values
    // This variable returns the name of the plane found in the aircraft.cfg (max 255 characters)
    conn.add_data_definition(1, "TITLE", "", simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING256, 4, 0.0);

    // Request the data from define_id 0 (floats) and only return the value if the value has changed including the id we passed in the datum_id
    // So if the latitude changes we receive: key 1 value X, if the longitude changes we receive key 2 value X.
    // If both have changed we receive both variables in an packed array.
    // The amount of variables returned is defined in the data.dwDefineCount of the response
    conn.request_data_on_sim_object(0, 0, 0, simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME, simconnect::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED | simconnect::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED, 0, 0, 0); //request_id, define_id, object_id (user), period, falgs, origin, interval, limit - tells simconnect to send data for the defined id and on the user aircraft
    // Request the data from our define_id 1 (strings)
    conn.request_data_on_sim_object(1, 1, 0, simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME, simconnect::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED | simconnect::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED, 0, 0, 0); //request_id, define_id, object_id (user), period, falgs, origin, interval, limit - tells simconnect to send data for the defined id and on the user aircraft

    loop {
        match conn.get_next_message() {
            Ok(simconnect::DispatchResult::SimobjectData(data)) => {
                unsafe {
                    match data.dwDefineID {
                        0 => {
                            let sim_data =  std::ptr::addr_of!(data.dwData);
                            let sim_data_ptr = sim_data as *const DataFloatStruct;
                            let sim_data_value = std::ptr::read_unaligned(sim_data_ptr);
                            // The amount of floats received from the sim
                            let count = data.dwDefineCount as usize;

                            // itterate through the array of data structs
                            // To allign the memory we have allocated an array of 255 elements to the datastruct
                            // The game might return 255 or 2 values
                            // To only itterate over valid elements in the array
                            // We are able to leverage the dwDefineCount to loop over valid elements
                            for i in 0..count {
                                let value = sim_data_value.data[i].value;
                                let key = sim_data_value.data[i].id;
                                println!("{}", key.to_string());
                                println!("{}", value);
                            }
                        },
                        1 => {
                            let sim_data =  std::ptr::addr_of!(data.dwData);
                            let sim_data_ptr = sim_data as *const DataStringStruct;
                            // The amount of strings received from the sim
                            let count = data.dwDefineCount as usize;
                            let sim_data_value = std::ptr::read_unaligned(sim_data_ptr);
                            for i in 0..count {
                                //since we only defined 1 string variable the key returned should be 4
                                let key = sim_data_value.data[0].id;
                                //byte array to string
                                let string = std::str::from_utf8(&sim_data_value.data[i].value).unwrap();
                                println!("{}", key.to_string());
                                println!("{}", string);
                            }

                        }
                        _ => ()
                    }
                }
            },
            _ => ()
        }

        sleep(Duration::from_millis(16)); // Will use up lots of CPU if this is not included, as get_next_message() is non-blocking
    }
}