use sensor::Sensor;
use observation_id::ObservationID;

use libc::{c_char, c_void};
use std::net::IpAddr;

#[no_mangle]
pub extern "C" fn sensor_new(ip: &[u8; 16]) -> *mut Sensor {
    let ip_addresss = IpAddr::from(*ip);
    Box::into_raw(Box::new(Sensor::new(ip_addresss)))
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

    let octets = match sensor.get_ip() {
        IpAddr::V4(ip) => ip.octets(),
        IpAddr::V6(ip) => ip.to_ipv4().expect("IPv6 are not supported for sensors").octets(),
    };

    24 << octets[0] + 16 << octets[1] + 8 << octets[2] + octets[3]
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

#[no_mangle]
pub extern "C" fn sensor_set_worker(sensor_ptr: *mut Sensor, worker: *const c_void) {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &mut *sensor_ptr
    };

    sensor.set_worker(worker);
}

#[no_mangle]
pub extern "C" fn sensor_add_observation_id(sensor_ptr: *mut Sensor,
                                            observation_id_ptr: *mut ObservationID) {
    let sensor = unsafe {
        assert!(!sensor_ptr.is_null());
        &mut *sensor_ptr
    };

    let observation_id = unsafe {
        assert!(!sensor_ptr.is_null());
        Box::from_raw(observation_id_ptr)
    };

    sensor.add_observations_id(*observation_id);
}

#[no_mangle]
pub extern "C" fn sensor_add_default_observation_id(sensor_ptr: *const Sensor,
                                                    id: u32,
                                                    observation_id: *mut ObservationID) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv6Addr;

    #[test]
    fn test_name() {
        let c_ip_address = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 168, 1, 1];
        let sensor = unsafe { &*sensor_new(&c_ip_address) };

        let ip_str: Ipv6Addr = "::192.168.1.1".parse().unwrap();
        let ip_address = sensor.get_ip();

        assert_eq!(ip_str, ip_address);
    }
}
