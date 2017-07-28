use observation_id::ObservationID;

use libc::{c_char, c_void, size_t};
use std::ffi::CStr;
use std::ptr;
use std::net::IpAddr;

#[no_mangle]
pub extern "C" fn observation_id_new(id: u32) -> *mut ObservationID {
    Box::into_raw(Box::new(ObservationID::new(id)))
}

#[no_mangle]
pub extern "C" fn observation_id_get_network_ip(
    observation_id_ptr: *const ObservationID,
    ip: [u8; 16],
) -> *const c_char {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    let ip_address = IpAddr::from(ip);

    match observation_id.get_network(ip_address) {
        Some(network) => network.get_address_str().as_ptr() as *const c_char,
        None => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn observation_id_get_id(observation_id_ptr: *const ObservationID) -> u32 {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    observation_id.get_id()
}

#[no_mangle]
pub extern "C" fn observation_id_get_network_name(
    observation_id_ptr: *const ObservationID,
    ip: [u8; 16],
) -> *const c_char {
    let observation_id = unsafe {
        assert!(!observation_id_ptr.is_null());
        &*observation_id_ptr
    };

    let ip_address = IpAddr::from(ip);

    match observation_id.get_network(ip_address) {
        Some(network) => {
            let name = network.get_name();
            name.as_ptr() as *const i8
        }
        None => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn observation_id_get_selector_name(
    observation_id_ptr: *const ObservationID,
    selector_id: u64,
) -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_interface_name(
    observation_id_ptr: *const ObservationID,
    interface_id: u64,
) -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_interface_description(
    observation_id_ptr: *const ObservationID,
    interface_id: u64,
) -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_get_fallback_first_switch(
    observation_id_ptr: *const ObservationID,
) -> i64 {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    match observation_id.get_fallback_first_switch() {
        Some(fallback_first_switch) => fallback_first_switch,
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn observation_id_list_templates(
    observation_id_ptr: *const ObservationID,
    len: *mut size_t,
) -> *mut u16 {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    let mut template_list = observation_id.list_templates().into_boxed_slice();
    unsafe { *len = template_list.len() as size_t };
    let sensor_list_raw = template_list.as_mut_ptr();

    Box::into_raw(template_list);
    sensor_list_raw
}

#[no_mangle]
pub extern "C" fn observation_id_get_template(
    observation_id_ptr: *const ObservationID,
    id: u16,
) -> *mut c_void {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    match observation_id.get_template(id) {
        Some(template) => *template,
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn observation_id_get_application_name(
    observation_id_ptr: *const ObservationID,
    application_id: u32,
) -> *const c_char {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    match observation_id.get_application(application_id) {
        Some(application) => application.get_name().as_ptr() as *const c_char,
        None => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn observation_id_get_enrichment(
    observation_id_ptr: *const ObservationID,
) -> *const c_char {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    match observation_id.get_enrichment() {
        Some(enrichment) => (*enrichment).as_ptr() as *const c_char,
        None => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn observation_id_want_client_dns(observation_id_ptr: *const ObservationID) -> bool {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    observation_id.want_client_dns()
}

#[no_mangle]
pub extern "C" fn observation_id_want_target_dns(observation_id_ptr: *const ObservationID) -> bool {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &*observation_id_ptr };

    observation_id.want_target_dns()
}

#[no_mangle]
pub extern "C" fn observation_id_is_exporter_in_wan_side(
    observation_id_ptr: *const ObservationID,
) -> bool {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_is_span(observation_id_ptr: *const ObservationID) -> bool {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_template(
    observation_id_ptr: *mut ObservationID,
    id: u16,
    template_ptr: *mut c_void,
) {
    assert!(!observation_id_ptr.is_null());
    assert!(!template_ptr.is_null());
    let mut observation_id = unsafe { &mut *observation_id_ptr };
    let template = unsafe { &mut *template_ptr };

    observation_id.add_template(id, template);
}

#[no_mangle]
pub extern "C" fn observation_id_add_template_async(
    observation_id_ptr: *const ObservationID,
    tmpl: *const c_void,
) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_application_id(
    observation_id_ptr: *const ObservationID,
    application_id: u64,
    application_name: *const c_char,
    application_name_len: size_t,
) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_selector_id(
    observation_id_ptr: *const ObservationID,
    selector_id: u64,
    selector_name: *const c_char,
    selector_name_len: size_t,
) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_add_interface(
    observation_id_ptr: *mut ObservationID,
    interface_id: u64,
    interface_name: *const c_char,
    interface_name_len: size_t,
    interface_description: *const c_char,
    interface_description_len: size_t,
) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_set_enrichment(
    observation_id_ptr: *mut ObservationID,
    enrichment_ptr: *mut c_char,
) {
    assert!(!observation_id_ptr.is_null());
    let observation_id = unsafe { &mut *observation_id_ptr };
    let enrichment = unsafe { CStr::from_ptr(enrichment_ptr) };
    observation_id.set_enrichment(enrichment.to_bytes_with_nul());
}

#[no_mangle]
pub extern "C" fn observation_id_set_exporter_in_wan_side(observation_id_ptr: *mut ObservationID) {
    let observation_id = unsafe {
        assert!(!observation_id_ptr.is_null());
        &mut *observation_id_ptr
    };

    observation_id.set_exporter_in_wan_side();
}

#[no_mangle]
pub extern "C" fn observation_id_set_span_mode(observation_id_ptr: *mut ObservationID) {
    unimplemented!()
}


#[no_mangle]
pub extern "C" fn observation_id_set_fallback_first_switch(
    observation_id_ptr: *mut ObservationID,
    fallback_first_switch: i64,
) {
    let observation_id = unsafe {
        assert!(!observation_id_ptr.is_null());
        &mut *observation_id_ptr
    };

    observation_id.set_fallback_first_switch(fallback_first_switch);
}

#[no_mangle]
pub extern "C" fn observation_id_enable_ptr_dns_client(observation_id_ptr: *mut ObservationID) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn observation_id_enable_ptr_dns_target(observation_id_ptr: *mut ObservationID) {
    let observation_id = unsafe {
        assert!(!observation_id_ptr.is_null());
        &mut *observation_id_ptr
    };

    observation_id.enable_ptr_dns_target();
}
