use simconnect::SIMCONNECT_CLIENT_EVENT_ID;
use std::collections::HashMap;

// define a struct that holds the event and the input id
// the events can be found in the SimConnect SDK documentation for your sim
struct Input {
    event: String, // The event to be triggered i.e.
    input_id: u32, // The id we use to trigger the event
}
fn main() {
    let input_parking_brakes = Input {
        event: "PARKING_BRAKES".to_string(),
        input_id: 1,
    };
    let input_gear_up = Input {
        event: "GEAR_UP".to_string(),
        input_id: 2,
    };
    let input_gear_down = Input {
        event: "GEAR_DOWN".to_string(),
        input_id: 3,
    };

    // Define a hashmap to easily cross reference the input id with the event
    let mut events: HashMap<u32, Input> = HashMap::new();
    events.insert(input_parking_brakes.input_id, input_parking_brakes);
    events.insert(input_gear_up.input_id, input_gear_up);
    events.insert(input_gear_down.input_id, input_gear_down);

    let mut conn = simconnect::SimConnector::new();
    conn.connect("Program that inputs commands to the sim"); // Initialize connection with SimConnect

    // loop over all the events we want to define and map them to the input id
    for event in &events {
        println!("Defining event: {}", event.1.event);
        println!("Input id: {}", event.1.input_id);
        conn.map_client_event_to_sim_event(
            // if input id 1 is triggered, the PARKING_BRAKES event is triggered
            event.1.input_id as SIMCONNECT_CLIENT_EVENT_ID,
            event.1.event.as_str(),
        );
    }
    // loop over user input from console and trigger the corresponding events
    loop {
        println!("Enter an input id to trigger an event");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input_id: u32 = input.trim().parse().unwrap();
        match events.get(&input_id) {
            // if the input id is found in the hashmap, trigger the event
            // otherwise print an error message
            Some(event) => {
                // this is why we've defined a hashmap instead of a vector
                println!("Triggering event: {}", event.event);
                /* send message to the sim
                object_id is 0 because we want to trigger the event on the user aircraft
                group_id is 0 because we don't want to group the event with other events
                priority is 0 because we don't want to prioritize the event
                this is used when multiple events are triggered at the same time
                */
                conn.transmit_client_event(
                    0,
                    input_id as u32,
                    0,
                    simconnect::SIMCONNECT_GROUP_PRIORITY_HIGHEST,
                    simconnect::SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
                );
            }
            None => println!("No event found for input id: {}", input_id),
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
