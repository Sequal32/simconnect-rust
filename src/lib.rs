#![allow(clippy::too_many_arguments, clippy::missing_safety_doc)]
/*!
The simconnect crate provides rust bindings to retrieve and send information through SimConnect.

Documentation for SimConnect can be found by downloading the SDK for FS2020 or using P3D/FSX SDK documentations for reference (although some of their documentation does not apply for FS2020).

# Setup
Add this to your `Cargo.toml`
```toml
[dependencies]
simconnect = "0.1"
```

# Simple Example
*Note: You must have SimConnect.dll in your current working directory to be able to successfully use SimConnect*
```rust
use simconnect;
use std::time::Duration;
use std::thread::sleep;
use std::mem::transmute_copy;

struct DataStruct {
  lat: f64,
  lon: f64,
  alt: f64,
}

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
    _ => ()
  }

  sleep(Duration::from_millis(16)); // Will use up lots of CPU if this is not included, as get_next_message() is non-blocking
}
```
!*/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::transmute_copy;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! as_c_string {
    ($target:expr) => {
        std::ffi::CString::new($target).unwrap().as_ptr()
    };
}

/// Enumerations for all the possible data types received from SimConnect
#[derive(Debug)]
pub enum DispatchResult<'a> {
    Null,
    Exception(&'a SIMCONNECT_RECV_EXCEPTION),
    Open(&'a SIMCONNECT_RECV_OPEN),
    Quit(&'a SIMCONNECT_RECV_QUIT),
    Event(&'a SIMCONNECT_RECV_EVENT),
    EventObjectAddRemove(&'a SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
    EventFilename(&'a SIMCONNECT_RECV_EVENT_FILENAME),
    EventFrame(&'a SIMCONNECT_RECV_EVENT_FRAME),
    SimobjectData(&'a SIMCONNECT_RECV_SIMOBJECT_DATA),
    SimobjectDataBytype(&'a SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE),
    WeatherObservation(&'a SIMCONNECT_RECV_WEATHER_OBSERVATION),
    CloudState(&'a SIMCONNECT_RECV_CLOUD_STATE),
    AssignedObjectId(&'a SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
    ReservedKey(&'a SIMCONNECT_RECV_RESERVED_KEY),
    CustomAction(&'a SIMCONNECT_RECV_CUSTOM_ACTION),
    SystemState(&'a SIMCONNECT_RECV_SYSTEM_STATE),
    ClientData(&'a SIMCONNECT_RECV_CLIENT_DATA),
    EventWeatherMode(&'a SIMCONNECT_RECV_EVENT_WEATHER_MODE),
    AirportList(&'a SIMCONNECT_RECV_AIRPORT_LIST),
    VorList(&'a SIMCONNECT_RECV_VOR_LIST),
    NdbList(&'a SIMCONNECT_RECV_NDB_LIST),
    WaypointList(&'a SIMCONNECT_RECV_WAYPOINT_LIST),
    EventMultiplayerServerStarted(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED),
    EventMultiplayerClientStarted(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED),
    EventMultiplayerSessionEnded(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED),
    EventRaceEnd(&'a SIMCONNECT_RECV_EVENT_RACE_END),
    EventRaceLap(&'a SIMCONNECT_RECV_EVENT_RACE_LAP),
}

/// Handles communication between the client program and SimConnect
/// For more information about the functions provided, refer to the SimConnect SDK Documentation. The functions name closely match up with those defined there.
pub struct SimConnector {
    sim_connect_handle: HANDLE,
}

impl Default for SimConnector {
    fn default() -> Self {
        Self {
            sim_connect_handle: std::ptr::null_mut(),
        }
    }
}

impl SimConnector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, program_name: &str) -> bool {
        unsafe {
            let temp_1 = ptr::null_mut();
            let temp_2 = ptr::null_mut();

            SimConnect_Open(
                &mut self.sim_connect_handle,
                as_c_string!(program_name),
                temp_1,
                0,
                temp_2,
                0,
            );

            !self.sim_connect_handle.is_null()
        }
    }

    pub fn add_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        datum_name: &str,
        units_name: &str,
        datum_type: SIMCONNECT_DATATYPE,
        datum_id: DWORD,
        epsilon: f32,
    ) -> bool {
        unsafe {
            SimConnect_AddToDataDefinition(
                self.sim_connect_handle,
                define_id,
                as_c_string!(datum_name),
                as_c_string!(units_name),
                datum_type,
                epsilon,
                datum_id,
            ) == 0
        }
    }

    pub fn set_system_event_state(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        state: SIMCONNECT_STATE,
    ) -> bool {
        unsafe { SimConnect_SetSystemEventState(self.sim_connect_handle, event_id, state) == 0 }
    }

    pub fn remove_client_event(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> bool {
        unsafe { SimConnect_RemoveClientEvent(self.sim_connect_handle, group_id, event_id) == 0 }
    }

    pub fn clear_notification_group(&self, group_id: SIMCONNECT_NOTIFICATION_GROUP_ID) -> bool {
        unsafe { SimConnect_ClearNotificationGroup(self.sim_connect_handle, group_id) == 0 }
    }

    pub fn request_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        reserved: DWORD,
        flags: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestNotificationGroup(self.sim_connect_handle, group_id, reserved, flags)
                == 0
        }
    }

    pub fn clear_data_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID) -> bool {
        unsafe { SimConnect_ClearDataDefinition(self.sim_connect_handle, define_id) == 0 }
    }

    pub fn create_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        size: DWORD,
        flags: SIMCONNECT_CREATE_CLIENT_DATA_FLAG,
    ) -> bool {
        unsafe { SimConnect_CreateClientData(self.sim_connect_handle, data_id, size, flags) == 0 }
    }

    pub fn request_data_on_sim_object_type(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        radius_in_meters: DWORD,
        object_type: SIMCONNECT_SIMOBJECT_TYPE,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObjectType(
                self.sim_connect_handle,
                request_id,
                define_id,
                radius_in_meters,
                object_type,
            ) == 0
        }
    }

    pub fn remove_input_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
    ) -> bool {
        unsafe {
            SimConnect_RemoveInputEvent(
                self.sim_connect_handle,
                group_id,
                as_c_string!(input_definition),
            ) == 0
        }
    }

    pub fn clear_input_group(&self, group_id: SIMCONNECT_INPUT_GROUP_ID) -> bool {
        unsafe { SimConnect_ClearInputGroup(self.sim_connect_handle, group_id) == 0 }
    }

    pub fn request_reserved_Key(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        key_choice_1: &str,
        key_choice_2: &str,
        key_choice_3: &str,
    ) -> bool {
        unsafe {
            SimConnect_RequestReservedKey(
                self.sim_connect_handle,
                event_id,
                as_c_string!(key_choice_1),
                as_c_string!(key_choice_2),
                as_c_string!(key_choice_3),
            ) == 0
        }
    }

    pub fn unsubscribe_from_system_event(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_UnsubscribeFromSystemEvent(self.sim_connect_handle, event_id) == 0 }
    }

    pub fn ai_create_parked_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        airport_id: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AICreateParkedATCAircraft(
                self.sim_connect_handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                as_c_string!(airport_id),
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_enroute_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        flight_number: i32,
        flight_plan_path: &str,
        flight_plan_position: f64,
        touch_and_go: bool,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AICreateEnrouteATCAircraft(
                self.sim_connect_handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                flight_number,
                as_c_string!(flight_plan_path),
                flight_plan_position,
                touch_and_go as i32,
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_non_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        init_pos: SIMCONNECT_DATA_INITPOSITION,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AICreateNonATCAircraft(
                self.sim_connect_handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                init_pos,
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_simulated_object(
        &self,
        container_title: &str,
        init_pos: SIMCONNECT_DATA_INITPOSITION,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AICreateSimulatedObject(
                self.sim_connect_handle,
                as_c_string!(container_title),
                init_pos,
                request_id,
            ) == 0
        }
    }

    pub fn ai_release_control(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_AIReleaseControl(self.sim_connect_handle, object_id, request_id) == 0 }
    }

    pub fn ai_remove_object(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_AIRemoveObject(self.sim_connect_handle, object_id, request_id) == 0 }
    }

    pub fn ai_set_aircraft_flight_plan(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        flight_plan_path: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AISetAircraftFlightPlan(
                self.sim_connect_handle,
                object_id,
                as_c_string!(flight_plan_path),
                request_id,
            ) == 0
        }
    }

    pub fn execute_mission_action(&self, instance_id: GUID) -> bool {
        unsafe { SimConnect_ExecuteMissionAction(self.sim_connect_handle, instance_id) == 0 }
    }

    pub fn complete_custom_mission_action(&self, instance_id: GUID) -> bool {
        unsafe { SimConnect_CompleteCustomMissionAction(self.sim_connect_handle, instance_id) == 0 }
    }

    pub fn close(&self) -> bool {
        unsafe { SimConnect_Close(self.sim_connect_handle) == 0 }
    }

    pub unsafe fn get_last_sent_packet_id(&self, error: *mut DWORD) -> bool {
        unsafe { SimConnect_GetLastSentPacketID(self.sim_connect_handle, error) == 0 }
    }

    // not tested
    pub unsafe fn call_dispatch(
        &self,
        dispatch_callback: DispatchProc,
        context: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe { SimConnect_CallDispatch(self.sim_connect_handle, dispatch_callback, context) == 0 }
    }

    pub unsafe fn request_response_times(&self, count: DWORD, elapsed_seconds: *mut f32) -> bool {
        unsafe {
            SimConnect_RequestResponseTimes(self.sim_connect_handle, count, elapsed_seconds) == 0
        }
    }

    pub fn camera_set_relative_6dof(
        &self,
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
        pitch: f32,
        bank: f32,
        heading: f32,
    ) -> bool {
        unsafe {
            SimConnect_CameraSetRelative6DOF(
                self.sim_connect_handle,
                delta_x,
                delta_y,
                delta_z,
                pitch,
                bank,
                heading,
            ) == 0
        }
    }

    pub fn menu_add_item(
        &self,
        menu_item: &str,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_MenuAddItem(
                self.sim_connect_handle,
                as_c_string!(menu_item),
                event_id,
                data,
            ) == 0
        }
    }

    pub fn menu_delete_item(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_MenuDeleteItem(self.sim_connect_handle, event_id) == 0 }
    }

    pub fn menu_delete_sub_item(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        sub_event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> bool {
        unsafe {
            SimConnect_MenuDeleteSubItem(self.sim_connect_handle, event_id, sub_event_id) == 0
        }
    }

    pub fn request_system_state(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        state: &str,
    ) -> bool {
        unsafe {
            SimConnect_RequestSystemState(self.sim_connect_handle, request_id, as_c_string!(state))
                == 0
        }
    }

    pub fn map_client_data_name_to_id(
        &self,
        client_data_name: &str,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
    ) -> bool {
        unsafe {
            SimConnect_MapClientDataNameToID(
                self.sim_connect_handle,
                as_c_string!(client_data_name),
                data_id,
            ) == 0
        }
    }

    pub fn add_to_client_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        offset: DWORD,
        size_or_type: DWORD,
        epsilon: f32,
        datum_id: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_AddToClientDataDefinition(
                self.sim_connect_handle,
                define_id,
                offset,
                size_or_type,
                epsilon,
                datum_id,
            ) == 0
        }
    }

    pub fn clear_client_data_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID) -> bool {
        unsafe { SimConnect_ClearClientDataDefinition(self.sim_connect_handle, define_id) == 0 }
    }

    pub fn request_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        period: SIMCONNECT_CLIENT_DATA_PERIOD,
        flags: SIMCONNECT_CLIENT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestClientData(
                self.sim_connect_handle,
                data_id,
                request_id,
                define_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub unsafe fn set_client_data(
        &self,
        data_id: SIMCONNECT_CLIENT_DATA_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        flags: DWORD,
        reserved: DWORD,
        unit_size: DWORD,
        data_set: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetClientData(
                self.sim_connect_handle,
                data_id,
                define_id,
                flags,
                reserved,
                unit_size,
                data_set,
            ) == 0
        }
    }

    pub fn flight_load(&self, file_name: &str) -> bool {
        unsafe { SimConnect_FlightLoad(self.sim_connect_handle, as_c_string!(file_name)) == 0 }
    }

    pub unsafe fn text(
        &self,
        text_type: SIMCONNECT_TEXT_TYPE,
        time_in_seconds: f32,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        unit_size: DWORD,
        data_set: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_Text(
                self.sim_connect_handle,
                text_type,
                time_in_seconds,
                event_id,
                unit_size,
                data_set,
            ) == 0
        }
    }

    pub unsafe fn subscribe_to_facilities(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_SubscribeToFacilities(self.sim_connect_handle, list_type, request_id) == 0
        }
    }

    pub fn unsubscribe_to_facilities(&self, list_type: SIMCONNECT_FACILITY_LIST_TYPE) -> bool {
        unsafe { SimConnect_UnsubscribeToFacilities(self.sim_connect_handle, list_type) == 0 }
    }

    pub fn request_facilities_list(
        &self,
        list_type: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_RequestFacilitiesList(self.sim_connect_handle, list_type, request_id) == 0
        }
    }

    pub fn request_data_on_sim_object(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        period: SIMCONNECT_CLIENT_DATA_PERIOD,
        flags: SIMCONNECT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObject(
                self.sim_connect_handle,
                request_id,
                define_id,
                object_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub unsafe fn set_data_on_sim_object(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        flags: SIMCONNECT_DATA_SET_FLAG,
        array_count: DWORD,
        size: DWORD,
        pntr: *mut ::std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetDataOnSimObject(
                self.sim_connect_handle,
                define_id,
                object_id,
                flags,
                array_count,
                size,
                pntr,
            ) == 0
        }
    }

    pub fn subscribe_to_system_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        unsafe {
            SimConnect_SubscribeToSystemEvent(
                self.sim_connect_handle,
                event_id,
                as_c_string!(event_name),
            ) == 0
        }
    }

    pub fn map_client_event_to_sim_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        unsafe {
            SimConnect_MapClientEventToSimEvent(
                self.sim_connect_handle,
                event_id,
                as_c_string!(event_name),
            ) == 0
        }
    }

    pub fn transmit_client_event(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        dw_data: DWORD,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        flags: SIMCONNECT_EVENT_FLAG,
    ) -> bool {
        unsafe {
            SimConnect_TransmitClientEvent(
                self.sim_connect_handle,
                object_id,
                event_id,
                dw_data,
                group_id,
                flags,
            ) == 0
        }
    }

    pub fn add_client_event_to_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        maskable: bool,
    ) -> bool {
        unsafe {
            SimConnect_AddClientEventToNotificationGroup(
                self.sim_connect_handle,
                group_id,
                event_id,
                maskable as i32,
            ) == 0
        }
    }

    pub fn set_notification_group_priority(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        priority: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_SetNotificationGroupPriority(self.sim_connect_handle, group_id, priority)
                == 0
        }
    }

    pub fn map_input_event_to_client_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
        down_event: SIMCONNECT_CLIENT_EVENT_ID,
        down_return_value: DWORD,
        up_event: SIMCONNECT_CLIENT_EVENT_ID,
        up_return_value: DWORD,
        maskable: bool,
    ) -> bool {
        unsafe {
            SimConnect_MapInputEventToClientEvent(
                self.sim_connect_handle,
                group_id,
                as_c_string!(input_definition),
                down_event,
                down_return_value,
                up_event,
                up_return_value,
                maskable as i32,
            ) == 0
        }
    }

    pub fn set_input_group_state(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, state: DWORD) -> bool {
        unsafe { SimConnect_SetInputGroupState(self.sim_connect_handle, group_id, state) == 0 }
    }

    pub fn set_input_priority(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, priority: DWORD) -> bool {
        unsafe {
            SimConnect_SetInputGroupPriority(self.sim_connect_handle, group_id, priority) == 0
        }
    }

    /// Retrieves the next message from SimConnect. Nonblocking.
    pub fn get_next_message(&self) -> Result<DispatchResult, &str> {
        let mut data_buf: *mut SIMCONNECT_RECV = ptr::null_mut();

        let mut size_buf: DWORD = 32;
        let size_buf_pointer: *mut DWORD = &mut size_buf;

        unsafe {
            let result = SimConnect_GetNextDispatch(
                self.sim_connect_handle,
                &mut data_buf,
                size_buf_pointer,
            );
            if result != 0 {
                return Err("Failed getting data!");
            }

            return match (*data_buf).dwID as SIMCONNECT_RECV_ID {
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => Ok(DispatchResult::Null),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => Ok(DispatchResult::Exception(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_EXCEPTION)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => Ok(DispatchResult::Open(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_OPEN)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Ok(DispatchResult::Quit(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_QUIT)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => Ok(DispatchResult::Event(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_EVENT)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_OBJECT_ADDREMOVE => {
                    Ok(DispatchResult::EventObjectAddRemove(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FILENAME => {
                    Ok(DispatchResult::EventFilename(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_FILENAME),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FRAME => {
                    Ok(DispatchResult::EventFrame(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_FRAME),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                    Ok(DispatchResult::SimobjectData(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SIMOBJECT_DATA),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA_BYTYPE => {
                    Ok(DispatchResult::SimobjectDataBytype(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WEATHER_OBSERVATION => {
                    Ok(DispatchResult::WeatherObservation(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_WEATHER_OBSERVATION),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLOUD_STATE => {
                    Ok(DispatchResult::CloudState(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CLOUD_STATE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ASSIGNED_OBJECT_ID => {
                    Ok(DispatchResult::AssignedObjectId(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_RESERVED_KEY => {
                    Ok(DispatchResult::ReservedKey(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_RESERVED_KEY),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CUSTOM_ACTION => {
                    Ok(DispatchResult::CustomAction(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CUSTOM_ACTION),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SYSTEM_STATE => {
                    Ok(DispatchResult::SystemState(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SYSTEM_STATE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLIENT_DATA => {
                    Ok(DispatchResult::ClientData(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CLIENT_DATA),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_WEATHER_MODE => {
                    Ok(DispatchResult::EventWeatherMode(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_WEATHER_MODE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => {
                    Ok(DispatchResult::AirportList(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_AIRPORT_LIST),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_VOR_LIST => Ok(DispatchResult::VorList(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_VOR_LIST)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NDB_LIST => Ok(DispatchResult::NdbList(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_NDB_LIST)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WAYPOINT_LIST => {
                    Ok(DispatchResult::WaypointList(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_WAYPOINT_LIST),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SERVER_STARTED => Ok(
                    DispatchResult::EventMultiplayerServerStarted(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_CLIENT_STARTED => Ok(
                    DispatchResult::EventMultiplayerClientStarted(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SESSION_ENDED => Ok(
                    DispatchResult::EventMultiplayerSessionEnded(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_END => {
                    Ok(DispatchResult::EventRaceEnd(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_RACE_END),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_LAP => {
                    Ok(DispatchResult::EventRaceLap(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_RACE_LAP),
                    )))
                }

                _ => Err("Unhandled RECV_ID"),
            };
        }
    }
}

impl Drop for SimConnector {
    fn drop(&mut self) {
        if !self.sim_connect_handle.is_null() {
            self.close();
        }
    }
}
