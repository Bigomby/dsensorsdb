use observation_id::ObservationID;

use libc::{c_char, c_void, size_t};

#[no_mangle]
pub extern "C" fn observation_id_get_network_ip(observation_id_ptr: *const ObservationID,
                                                ip: [u8; 16])
                                                -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_id(observation_id_ptr: *const ObservationID) -> u32 {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_network_name(observation_id_ptr: *const ObservationID,
                                                  ip: [u8; 16])
                                                  -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_selector_name(observation_id_ptr: *const ObservationID,
                                                   selector_id: u64)
                                                   -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_interface_name(observation_id_ptr: *const ObservationID,
                                                    interface_id: u64)
                                                    -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_interface_description(observation_id_ptr: *const ObservationID,
                                          interface_id:u64)-> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_fallback_first_switch(observation_id_ptr: *const ObservationID) -> i64 {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_template(observation_id_ptr: *const ObservationID,
                                              template_id: u16)
                                              -> *mut c_void {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_application_name(observation_id_ptr: *const ObservationID,
                                                      application_id: u64)
                                                      -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_enrichment(observation_id_ptr: *const ObservationID)
                                                -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_want_client_dns(observation_id_ptr: *const ObservationID) -> bool {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_want_target_dns(observation_id_ptr: *const ObservationID) -> bool {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_template(observation_id_ptr: *const ObservationID,
                                              tmpl: *const c_void) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_template_async(observation_id_ptr: *const ObservationID,
                                                    tmpl: *const c_void) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_application_id(observation_id_ptr: *const ObservationID,
                                                    application_id: u64,
                                                    application_name: *const c_char,
                                                    application_name_len: size_t) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_selector_id(observation_id_ptr: *const ObservationID,
                                                 selector_id: u64,
                                                 selector_name: *const c_char,
                                                 selector_name_len: size_t) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_interface(observation_id_ptr: *const ObservationID,
                                               interface_id: u64,
                                               interface_name: *const c_char,
                                               interface_name_len: size_t,
                                               interface_description: *const c_char,
                                               interface_description_len: size_t) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_is_exporter_in_wan_side(observation_id_ptr: *const ObservationID)
                                                         -> bool {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_is_span(observation_id_ptr: *const ObservationID) -> bool {
    unimplemented!()
}
