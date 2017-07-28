use sensor::Sensor;
use observation_id::ObservationID;

use libc::{c_char, c_void, size_t};
use std::ptr;

#[no_mangle]
pub extern "C" fn sensor_new(ip: &[u8; 16]) -> *mut Sensor {
    if ip[11] != 0xFF || ip[10] != 0xFF {
        panic!("Unsupported IPv6 address")
    }

    let a = (ip[12] as u32) << 24;
    let b = (ip[13] as u32) << 16;
    let c = (ip[14] as u32) << 8;
    let d = ip[15] as u32;

    Box::into_raw(Box::new(Sensor::new(a + b + c + d)))
}

#[no_mangle]
pub extern "C" fn sensor_get_ip_string(sensor_ptr: *const Sensor) -> *const c_char {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &*sensor_ptr
    };

    sensor.get_ip_string().as_ptr() as *const i8
}

#[no_mangle]
pub extern "C" fn sensor_get_ip(sensor_ptr: *const Sensor) -> u32 {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &*sensor_ptr
    };

    sensor.get_ip()
}

#[no_mangle]
pub extern "C" fn sensor_get_observation_id_list(
    sensor_ptr: *const Sensor,
    len: *mut size_t,
) -> *mut u32 {
    assert!(!sensor_ptr.is_null());
    let sensor = unsafe { &*sensor_ptr };

    let mut observation_id_list = sensor.list_observation_ids().into_boxed_slice();
    unsafe { *len = observation_id_list.len() as size_t };
    let sensor_list_raw = observation_id_list.as_mut_ptr();

    Box::into_raw(observation_id_list);
    sensor_list_raw
}

#[no_mangle]
pub extern "C" fn sensor_get_observation_id(
    sensor_ptr: *mut Sensor,
    id: u32,
) -> *mut ObservationID {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &mut *sensor_ptr
    };

    match sensor.get_observation_id(id) {
        Some(observation_id) => observation_id as *mut ObservationID,
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn sensor_get_worker(sensor_ptr: *const Sensor) -> *mut c_void {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &*sensor_ptr
    };

    sensor.get_worker().expect(
        "No worker associated to the sensor",
    )
}

#[no_mangle]
pub extern "C" fn sensor_set_worker(sensor_ptr: *mut Sensor, worker: *mut c_void) {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &mut *sensor_ptr
    };

    sensor.set_worker(worker);
}

#[no_mangle]
pub extern "C" fn sensor_add_observation_id(
    sensor_ptr: *mut Sensor,
    observation_id_ptr: *mut ObservationID,
) {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &mut *sensor_ptr
    };

    let observation_id = unsafe {
        assert!(!observation_id_ptr.is_null());
        Box::from_raw(observation_id_ptr)
    };

    sensor.add_observation_id(*observation_id);
}

#[no_mangle]
pub extern "C" fn sensor_add_default_observation_id(
    sensor_ptr: *mut Sensor,
    observation_id_ptr: *mut ObservationID,
) {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &mut *sensor_ptr
    };

    let observation_id = unsafe {
        assert!(!observation_id_ptr.is_null());
        Box::from_raw(observation_id_ptr)
    };

    sensor.add_default_observation_id(*observation_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_new() {
        let c_ip_address: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 192, 168, 1, 1];
        let sensor = unsafe { &*sensor_new(&c_ip_address) };
        let ip_address = sensor.get_ip_string();

        assert_eq!("192.168.1.1", ip_address);
    }
}
