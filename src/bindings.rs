use SensorsDB;
use Sensor;
use std::ptr;
use network::NetAddress;

// SensorsDB

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
pub extern "C" fn sensors_db_add(database_ptr: *mut SensorsDB, ip: u32, sensor_ptr: *mut Sensor) {
    let database = unsafe {
        assert!(!database_ptr.is_null());
        &mut *database_ptr
    };
    if sensor_ptr.is_null() {
        return;
    }

    let sensor = unsafe { Box::from_raw(sensor_ptr) };

    database.add_sensor(ip, *sensor);
}

#[no_mangle]
pub extern "C" fn sensors_db_get(ptr: *const SensorsDB, ip: u32) -> *const Sensor {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    match database.get_sensor(ip) {
        Some(sensor) => &*sensor,
        None => ptr::null(),
    }
}

// Sensor

#[no_mangle]
pub extern "C" fn sensor_new(ip: NetAddress) -> *mut Sensor {
    Box::into_raw(Box::new(Sensor::new(ip)))
}

#[no_mangle]
pub extern "C" fn sensor_get_address(sensor_ptr: *const Sensor) -> NetAddress {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &*sensor_ptr
    };

    sensor.get_address()
}
