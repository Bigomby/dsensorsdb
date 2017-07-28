#[no_mangle]
pub extern "C" fn template_list_free(list_ptr: *mut u16) {
    if list_ptr.is_null() {
        return;
    };

    unsafe { Box::from_raw(list_ptr) };
}

#[no_mangle]
pub extern "C" fn observation_id_list_free(list_ptr: *mut u32) {
    if list_ptr.is_null() {
        return;
    };

    unsafe { Box::from_raw(list_ptr) };
}

#[no_mangle]
pub extern "C" fn sensor_list_free(list_ptr: *mut u32) {
    if list_ptr.is_null() {
        return;
    };

    unsafe { Box::from_raw(list_ptr) };
}
