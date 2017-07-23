use database::SensorsDB;
use sensor::Sensor;

use libc::c_int;
use std::ptr;
use std::net::{IpAddr, Ipv4Addr};

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
pub extern "C" fn sensors_db_get(database_ptr: *const SensorsDB, ip: u32) -> *const Sensor {
    let database = unsafe {
        assert!(!database_ptr.is_null());
        &*database_ptr
    };

    let ip_addresss = IpAddr::from(Ipv4Addr::from(ip).to_ipv6_mapped());

    match database.get_sensor(ip_addresss) {
        Some(sensor) => &*sensor,
        None => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn sensors_db_add(database_ptr: *mut SensorsDB, sensor_ptr: *mut Sensor) {
    let database = unsafe {
        assert!(!database_ptr.is_null());
        &mut *database_ptr
    };
    if sensor_ptr.is_null() {
        return;
    }

    let sensor = unsafe { Box::from_raw(sensor_ptr) };

    database.add_sensor(*sensor);
}

#[no_mangle]
pub extern "C" fn sensors_db_add_bad_sensor(database_ptr: *mut SensorsDB, sensor_ip: u64) -> c_int {
    unimplemented!()
}
