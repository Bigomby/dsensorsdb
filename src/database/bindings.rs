use database::SensorsDB;
use sensor::Sensor;

use libc::size_t;
use std::ptr;

#[no_mangle]
pub extern "C" fn sensors_db_new() -> *mut SensorsDB {
    Box::into_raw(Box::new(SensorsDB::new()))
}

#[no_mangle]
pub extern "C" fn sensors_db_destroy(database_ptr: *mut SensorsDB) {
    if database_ptr.is_null() {
        return;
    }

    unsafe { Box::from_raw(database_ptr) };
}

#[no_mangle]
pub extern "C" fn sensors_db_list(database_ptr: *const SensorsDB, len: *mut size_t) -> *mut u32 {
    assert!(!database_ptr.is_null());
    let database = unsafe { &*database_ptr };

    let mut sensor_list = database.list_sensors().into_boxed_slice();
    unsafe { *len = sensor_list.len() as size_t };
    let sensor_list_raw = sensor_list.as_mut_ptr();

    Box::into_raw(sensor_list);
    sensor_list_raw
}

#[no_mangle]
pub extern "C" fn sensors_db_get(database_ptr: *const SensorsDB, ip: u32) -> *const Sensor {
    assert!(!database_ptr.is_null());
    let database = unsafe { &*database_ptr };

    match database.get_sensor(ip) {
        Some(sensor) => &*sensor,
        None => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn sensors_db_add(database_ptr: *mut SensorsDB, sensor_ptr: *mut Sensor) {
    assert!(!database_ptr.is_null());
    let database = unsafe { &mut *database_ptr };
    if sensor_ptr.is_null() {
        return;
    }

    let sensor = unsafe { Box::from_raw(sensor_ptr) };

    database.add_sensor(*sensor);
}
