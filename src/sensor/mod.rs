pub mod bindings;

use observation_id::ObservationID;
use std::collections::HashMap;
use std::net::IpAddr;
use libc::c_void;

pub struct Sensor {
    address: IpAddr,
    str_address: String,
    worker: Option<*mut c_void>,
    default_observation_id: Option<ObservationID>,
    observation_id: HashMap<u32, ObservationID>,
}

impl Sensor {
    pub fn new(address: IpAddr) -> Self {
        Sensor {
            address: address,
            str_address: format!("{}", address),
            worker: None,
            default_observation_id: None,
            observation_id: HashMap::new(),
        }
    }

    pub fn get_ip(&self) -> IpAddr {
        self.address
    }

    pub fn get_ip_string(&self) -> &str {
        &self.str_address
    }

    pub fn get_worker(&self) -> Option<*mut c_void> {
        self.worker
    }

    pub fn get_observation_id(&mut self, id: u32) -> Option<&mut ObservationID> {
        self.observation_id.get_mut(&id)
    }

    pub fn get_default_observation_id(&mut self) -> Option<&mut ObservationID> {
        self.default_observation_id.as_mut()
    }

    pub fn set_worker(&mut self, worker: *mut c_void) {
        self.worker = Some(worker);
    }

    pub fn add_observation_id(&mut self, observation_id: ObservationID) {
        self.observation_id.insert(observation_id.get_id(), observation_id);
    }

    pub fn add_default_observation_id(&mut self, observation_id: ObservationID) {
        self.default_observation_id = Some(observation_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv6Addr;

    static TEST_NETWORK: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 168, 1, 1];

    #[test]
    fn create_sensor_with_ipv4() {
        let sensor = Sensor::new(IpAddr::from(TEST_NETWORK));
        let ip_address = sensor.get_ip();
        let ip_str: Ipv6Addr = "::192.168.1.1".parse().unwrap();

        assert_eq!(ip_str, ip_address);
    }

    #[test]
    fn add_observation_ids() {
        let mut sensor = Sensor::new(IpAddr::from(TEST_NETWORK));

        let observation_id_1 = ObservationID::new(0);
        let observation_id_2 = ObservationID::new(123);

        sensor.add_default_observation_id(observation_id_1);
        sensor.add_observation_id(observation_id_2);

        assert!(sensor.get_default_observation_id().is_some());
        assert!(sensor.get_observation_id(123).is_some());
        assert!(sensor.get_observation_id(456).is_none());
    }
}
