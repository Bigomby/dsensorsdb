pub mod bindings;

use observation_id::ObservationID;
use std::collections::HashMap;
use libc::c_void;
use std::net::Ipv4Addr;

pub struct Sensor {
    address: u32,
    str_address: String,
    worker: Option<*mut c_void>,
    default_observation_id: Option<ObservationID>,
    observation_id: HashMap<u32, ObservationID>,
}

impl Sensor {
    pub fn new(address: u32) -> Self {
        Sensor {
            address: address,
            str_address: format!("{}", Ipv4Addr::from(address)),
            worker: None,
            default_observation_id: None,
            observation_id: HashMap::new(),
        }
    }

    pub fn get_ip(&self) -> u32 {
        self.address
    }

    pub fn get_ip_string(&self) -> &str {
        &self.str_address
    }

    pub fn get_worker(&self) -> Option<*mut c_void> {
        self.worker
    }

    pub fn list_observation_ids(&self) -> Vec<u32> {
        let mut observation_id_list = Vec::new();
        for observation_id in self.observation_id.keys() {
            observation_id_list.push(*observation_id);
        }

        match self.default_observation_id {
            Some(ref default_observation_id) => {
                observation_id_list.push(default_observation_id.get_id())
            }
            None => {}
        }

        observation_id_list
    }

    pub fn get_observation_id(&mut self, id: u32) -> Option<&mut ObservationID> {
        match self.observation_id.get_mut(&id) {
            Some(observation_id) => Some(observation_id),
            None => self.default_observation_id.as_mut(),
        }
    }

    pub fn set_worker(&mut self, worker: *mut c_void) {
        self.worker = Some(worker);
    }

    pub fn add_observation_id(&mut self, observation_id: ObservationID) {
        self.observation_id.insert(
            observation_id.get_id(),
            observation_id,
        );
    }

    pub fn add_default_observation_id(&mut self, observation_id: ObservationID) {
        self.default_observation_id = Some(observation_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv6Addr;

    // #[test]
    fn create_sensor_with_ipv4() {
        let sensor = Sensor::new(1234);
        let ip_address = sensor.get_ip();

        assert_eq!(123, ip_address);
    }

    #[test]
    fn add_observation_ids() {
        let mut sensor = Sensor::new(1234);

        let observation_id_1 = ObservationID::new(0);
        let observation_id_2 = ObservationID::new(123);

        sensor.add_default_observation_id(observation_id_1);
        sensor.add_observation_id(observation_id_2);

        assert_eq!(sensor.get_observation_id(0).unwrap().get_id(), 0);
        assert_eq!(sensor.get_observation_id(123).unwrap().get_id(), 123);
        assert_eq!(sensor.get_observation_id(456).unwrap().get_id(), 0);
    }
}
