#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ptr;
use std::mem;

include!("../bindings.rs"); 

macro_rules! as_c_string {
    ($target:expr) => {
        std::ffi::CString::new($target).unwrap().as_ptr();
    };
}

#[derive(Debug)]
pub enum DispatchResult {
    Null,
    Exception(*const SIMCONNECT_RECV_EXCEPTION),
    Open(*const SIMCONNECT_RECV_OPEN),
    Quit(*const SIMCONNECT_RECV_QUIT),
    Event(*const SIMCONNECT_RECV_EVENT),
    EventObjectAddRemove(*const SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
    EventFilename(*const SIMCONNECT_RECV_EVENT_FILENAME),
    EventFrame(*const SIMCONNECT_RECV_EVENT_FRAME),
    SimobjectData(*const SIMCONNECT_RECV_SIMOBJECT_DATA),
    SimobjectDataBytype(*const SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE),
    WeatherObservation(*const SIMCONNECT_RECV_WEATHER_OBSERVATION),
    CloudState(*const SIMCONNECT_RECV_CLOUD_STATE),
    AssignedObjectId(*const SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
    ReservedKey(*const SIMCONNECT_RECV_RESERVED_KEY),
    CustomAction(*const SIMCONNECT_RECV_CUSTOM_ACTION),
    SystemState(*const SIMCONNECT_RECV_SYSTEM_STATE),
    ClientData(*const SIMCONNECT_RECV_CLIENT_DATA),
    EventWeatherMode(*const SIMCONNECT_RECV_EVENT_WEATHER_MODE),
    AirportList(*const SIMCONNECT_RECV_AIRPORT_LIST),
    VorList(*const SIMCONNECT_RECV_VOR_LIST),
    NdbList(*const SIMCONNECT_RECV_NDB_LIST),
    WaypointList(*const SIMCONNECT_RECV_WAYPOINT_LIST),
    EventMultiplayerServerStarted(*const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED),
    EventMultiplayerClientStarted(*const SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED),
    EventMultiplayerSessionEnded(*const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED),
    EventRaceEnd(*const SIMCONNECT_RECV_EVENT_RACE_END),
    EventRaceLap(*const SIMCONNECT_RECV_EVENT_RACE_LAP),
}

pub struct SimConnector {
    sim_connect_handle: HANDLE
}

impl SimConnector {
    pub fn new() -> Self {
        Self {
            sim_connect_handle: std::ptr::null_mut()
        }
    }

    pub fn connect(&mut self, program_name: &str) -> bool {
        unsafe {
            SimConnect_Open(&mut self.sim_connect_handle, as_c_string!(program_name), ptr::null_mut(), 0, std::ptr::null_mut(), 0);
            return !self.sim_connect_handle.is_null();
        }
    }

    pub fn add_data_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID, datum_name: &str, units_name: &str, datum_type: SIMCONNECT_DATATYPE, datum_id: u32) -> bool {
        unsafe {
            let result = SimConnect_AddToDataDefinition(self.sim_connect_handle, define_id, as_c_string!(datum_name), as_c_string!(units_name), datum_type, 0.0, datum_id);
            return result == 0;
        }
    }

    pub fn request_data_on_sim_object(&self, request_id: SIMCONNECT_DATA_REQUEST_ID, define_id: SIMCONNECT_DATA_DEFINITION_ID, object_id: SIMCONNECT_OBJECT_ID, period: SIMCONNECT_CLIENT_DATA_PERIOD) -> bool {
        unsafe {
            let result = SimConnect_RequestDataOnSimObject(self.sim_connect_handle, request_id, define_id, object_id, period, 0, 0, 0, 0);
            return result == 0;
        }
    }

    pub fn set_data_on_sim_object(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID, object_id: SIMCONNECT_OBJECT_ID, flags: SIMCONNECT_DATA_SET_FLAG, array_count: DWORD, size: DWORD, pntr: *mut ::std::os::raw::c_void) -> bool {
        unsafe {
            let result = SimConnect_SetDataOnSimObject(self.sim_connect_handle, define_id, object_id, flags, array_count, size, pntr);
            return result == 0;
        }
    }

    pub fn subcribe_to_system_event(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID, event_name: &str) -> bool {
        unsafe {
            let result = SimConnect_SubscribeToSystemEvent(self.sim_connect_handle, event_id, as_c_string!(event_name));
            return result == 0;
        }
    }

    pub fn map_client_event_to_sim_event(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID, event_name: &str) -> bool {
        unsafe {
            let result = SimConnect_MapClientEventToSimEvent(self.sim_connect_handle, event_id, as_c_string!(event_name));
            return result == 0;
        }
    }

    pub fn transmit_client_event(&self, object_id: SIMCONNECT_OBJECT_ID, event_id: SIMCONNECT_CLIENT_EVENT_ID, dw_data: DWORD, group_id: SIMCONNECT_NOTIFICATION_GROUP_ID, flags: SIMCONNECT_EVENT_FLAG) -> bool{
        unsafe {
            let result = SimConnect_TransmitClientEvent(self.sim_connect_handle, object_id, event_id, dw_data, group_id, flags);
            return result == 0;
        }
    }

    pub fn add_client_event_to_notification_group(&self, group_id: SIMCONNECT_NOTIFICATION_GROUP_ID, event_id: SIMCONNECT_CLIENT_EVENT_ID, maskable: bool) -> bool {
        unsafe {
            let result = SimConnect_AddClientEventToNotificationGroup(self.sim_connect_handle, group_id, event_id, maskable as i32);
            return result == 0;
        }
    }

    pub fn set_notification_group_priority(&self, group_id: SIMCONNECT_NOTIFICATION_GROUP_ID, priority: DWORD) -> bool {
        unsafe {
            let result = SimConnect_SetNotificationGroupPriority(self.sim_connect_handle, group_id, priority);
            return result == 0;
        }
    }

    pub fn map_input_event_to_client_event(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, input_definition: &str, down_event: SIMCONNECT_CLIENT_EVENT_ID, down_return_value: DWORD, up_event: SIMCONNECT_CLIENT_EVENT_ID, up_return_value: DWORD, maskable: bool) -> bool {
        unsafe {
            let result = SimConnect_MapInputEventToClientEvent(self.sim_connect_handle, group_id, as_c_string!(input_definition), down_event, down_return_value, up_event, up_return_value, maskable as i32);
            return result == 0;
        }
    }

    pub fn set_input_group_state(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, state: DWORD) -> bool {
        unsafe {
            let result = SimConnect_SetInputGroupState(self.sim_connect_handle, group_id, state);
            return result == 0;
        }
    }

    pub fn set_input_priority(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, priority: DWORD) -> bool {
        unsafe {
            let result = SimConnect_SetInputGroupPriority(self.sim_connect_handle, group_id, priority);
            return result == 0;
        }
    }

    pub fn get_next_message(&self) -> Result<DispatchResult, &str> {
        let mut data_buf: *mut SIMCONNECT_RECV = ptr::null_mut();

        let mut size_buf: DWORD = 32;
        let size_buf_pointer: *mut DWORD = &mut size_buf;

        unsafe {
            SimConnect_GetNextDispatch(self.sim_connect_handle, &mut data_buf, size_buf_pointer);
            if data_buf.is_null() {return Err("Failed getting data!");}

            return match (*data_buf).dwID as SIMCONNECT_RECV_ID {
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => Ok(DispatchResult::Null),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => {
                    let point:*const SIMCONNECT_RECV_EXCEPTION = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::Exception(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
                    let point:*const SIMCONNECT_RECV_OPEN = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::Open(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => {
                    let point:*const SIMCONNECT_RECV_QUIT = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::Quit(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
                    let point:*const SIMCONNECT_RECV_EVENT = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::Event(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_OBJECT_ADDREMOVE => {
                    let point:*const SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventObjectAddRemove(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FILENAME => {
                    let point:*const SIMCONNECT_RECV_EVENT_FILENAME = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventFilename(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FRAME => {
                    let point:*const SIMCONNECT_RECV_EVENT_FRAME = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventFrame(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                    let point:*const SIMCONNECT_RECV_SIMOBJECT_DATA = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::SimobjectData(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA_BYTYPE => {
                    let point:*const SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::SimobjectDataBytype(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WEATHER_OBSERVATION  => {
                    let point:*const SIMCONNECT_RECV_WEATHER_OBSERVATION = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::WeatherObservation(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLOUD_STATE  => {
                    let point:*const SIMCONNECT_RECV_CLOUD_STATE = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::CloudState(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ASSIGNED_OBJECT_ID  => {
                    let point:*const SIMCONNECT_RECV_ASSIGNED_OBJECT_ID = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::AssignedObjectId(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_RESERVED_KEY  => {
                    let point:*const SIMCONNECT_RECV_RESERVED_KEY = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::ReservedKey(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CUSTOM_ACTION  => {
                    let point:*const SIMCONNECT_RECV_CUSTOM_ACTION = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::CustomAction(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SYSTEM_STATE  => {
                    let point:*const SIMCONNECT_RECV_SYSTEM_STATE = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::SystemState(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLIENT_DATA  => {
                    let point:*const SIMCONNECT_RECV_CLIENT_DATA = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::ClientData(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_WEATHER_MODE  => {
                    let point:*const SIMCONNECT_RECV_EVENT_WEATHER_MODE = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventWeatherMode(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST  => {
                    let point:*const SIMCONNECT_RECV_AIRPORT_LIST = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::AirportList(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_VOR_LIST  => {
                    let point:*const SIMCONNECT_RECV_VOR_LIST = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::VorList(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NDB_LIST  => {
                    let point:*const SIMCONNECT_RECV_NDB_LIST = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::NdbList(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WAYPOINT_LIST  => {
                    let point:*const SIMCONNECT_RECV_WAYPOINT_LIST = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::WaypointList(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SERVER_STARTED => {
                    let point:*const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventMultiplayerServerStarted(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_CLIENT_STARTED => {
                    let point:*const SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventMultiplayerClientStarted(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SESSION_ENDED => {
                    let point:*const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventMultiplayerSessionEnded(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_END  => {
                    let point:*const SIMCONNECT_RECV_EVENT_RACE_END = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventRaceEnd(point));
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_LAP  => {
                    let point:*const SIMCONNECT_RECV_EVENT_RACE_LAP = mem::transmute_copy(&data_buf);
                    return Ok(DispatchResult::EventRaceLap(point));
                }
                _ => Err("Unhandled RECV_ID")
            }
        }
    }
}