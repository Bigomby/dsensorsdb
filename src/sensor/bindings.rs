use network::NetAddress;
use sensor::Sensor;
use observation_id::ObservationID;

use libc::{c_char, c_void};

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

#[no_mangle]
pub extern "C" fn sensor_get_ip_string(sensor_ptr: *const Sensor) -> *const c_char {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sensor_get_observation_id(sensor_ptr: *const Sensor,
                                            id: u32)
                                            -> *mut ObservationID {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn sensor_get_worker(sensor_ptr: *const Sensor) -> *mut c_void {
    unimplemented!()
}
