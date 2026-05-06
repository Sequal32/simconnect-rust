use simconnect::{DispatchResult, DWORD};
use std::ptr::read_unaligned;
use std::thread::sleep;
use std::time::Duration;

// To allign the memory we have to set a fixed max size to the returned variables from the game
const MAX_RETURNED_ITEMS: usize = 255;

// Rust will add padding to the inner parts of a struct if it isn't marked as packed
// The way Simconnect returns values is unaligned data in C style
#[repr(C, packed)]
struct KeyValuePairFloat {
    id: DWORD,
    value: f64,
}

#[repr(C, packed)]
struct KeyValuePairString {
    id: DWORD,
    // Strings get returned as max 255 bytes
    value: [u8; MAX_RETURNED_ITEMS],
}

fn main() {
    let mut conn = simconnect::SimConnector::new();
    conn.connect("Program that returns data on changes"); // Initialize connection with SimConnect

    // Here we define all our variable that get returned as floats
    // (including integers, which the memory alignment will handle)
    // The epsilon determines per X change do we want to receive an update from the game
    // This greatly reduces the amount of data send to your client
    // In this example the lat, lon values get an update every degree while the altitude only gets an
    // update every 100 feet
    conn.add_data_definition(
        0,
        "PLANE LATITUDE",
        "Degrees",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
        1,
        1.0,
    ); // Assign a sim variable to a client defined id
    conn.add_data_definition(
        0,
        "PLANE LONGITUDE",
        "Degrees",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
        2,
        1.0,
    );
    conn.add_data_definition(
        0,
        "PLANE ALTITUDE",
        "Feet",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
        3,
        100.0,
    ); //define_id, units, data_type, datum_id, epsilon (update threshold)

    // Here we define all our variabes that get returned as Strings
    // Notice how the define_id differs from the float values
    // This variable returns the name of the plane found in the aircraft.cfg (max 255 characters)
    conn.add_data_definition(
        1,
        "TITLE",
        "",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING256,
        4,
        0.0,
    );

    // Request the data from define_id 0 (floats) and only return the value if the value has changed including the id we passed in the datum_id
    // So if the latitude changes we receive: key 1 value X, if the longitude changes we receive key 2 value X.
    // If both have changed we receive both variables in an packed array.
    // The amount of variables returned is defined in the data.dwDefineCount of the response
    conn.request_data_on_sim_object(
        0,
        0,
        0,
        simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
        simconnect::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED
            | simconnect::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED,
        0,
        0,
        0,
    ); //request_id, define_id, object_id (user), period, falgs, origin, interval, limit - tells simconnect to send data for the defined id and on the user aircraft
       // Request the data from our define_id 1 (strings)
       // The request_id has to differ from the float request. Or else it will overwrite the previous request
    conn.request_data_on_sim_object(
        1,
        1,
        0,
        simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
        simconnect::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED
            | simconnect::SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED,
        0,
        0,
        0,
    ); //request_id, define_id, object_id (user), period, falgs, origin, interval, limit - tells simconnect to send data for the defined id and on the user aircraft

    loop {
        match conn.get_next_message() {
            Ok(DispatchResult::SimObjectData(data)) => unsafe {
                match data.dwDefineID {
                    // Here we match the define_id we've passed using the request_data_on_sim_object
                    0 => {
                        let sim_data_ptr =
                            std::ptr::addr_of!(data.dwData) as *const KeyValuePairFloat;
                        // The amount of floats received from the sim
                        let count = data.dwDefineCount as usize;

                        // Only read the values that SimConnect tells us were returned.
                        for i in 0..count {
                            let item = read_unaligned(sim_data_ptr.add(i));
                            let id = read_unaligned(std::ptr::addr_of!(item.id));
                            let value = read_unaligned(std::ptr::addr_of!(item.value));
                            println!("{}", id);
                            println!("{}", value);
                        }
                    }
                    1 => {
                        let sim_data_ptr =
                            std::ptr::addr_of!(data.dwData) as *const KeyValuePairString;
                        // The amount of strings received from the sim
                        let count = data.dwDefineCount as usize;
                        for i in 0..count {
                            let item = read_unaligned(sim_data_ptr.add(i));
                            let id = read_unaligned(std::ptr::addr_of!(item.id));
                            let bytes: [u8; MAX_RETURNED_ITEMS] =
                                read_unaligned(std::ptr::addr_of!(item.value));
                            // Byte array to string, trimmed at the first NUL.
                            let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                            let string =
                                std::str::from_utf8(&bytes[..end]).unwrap_or("<invalid utf-8>");
                            println!("{}", id);
                            println!("{}", string);
                        }
                    }
                    _ => (),
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
