use observation_id::ObservationID;
use network::{NetAddress, IPAddress};
use std::collections::HashMap;

pub struct Sensor {
    address: NetAddress,
    observations_id: HashMap<u32, ObservationID>,
}

impl Sensor {
    pub fn new(address: NetAddress) -> Self {
        Sensor {
            address: address,
            observations_id: HashMap::new(),
        }
    }

    pub fn get_address(&self) -> NetAddress {
        self.address
    }

    fn get_observation_id(&self, id: u32) -> Option<&ObservationID> {
        self.observations_id.get(&id)
    }

    fn add_observations_id(&mut self, id: u32, observation_id: ObservationID) {
        self.observations_id.insert(id, observation_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_NETWORK: IPAddress = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
                                      0x01, 0x00, 0x01, 0x02, 0x03, 0x04];
    static TEST_NETWORK_MASK: IPAddress = [0x0, 0x0, 0x0, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05,
                                           0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    static TEST_BROADCAST: IPAddress = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    #[test]
    fn create_sensor() {
        let home_net = NetAddress::new(TEST_NETWORK, TEST_NETWORK_MASK, TEST_BROADCAST);
        let sensor = Sensor::new(home_net);
        let address = sensor.get_address();

        assert_eq!(address, home_net);
    }

    #[test]
    fn add_observations_ids() {
        let home_net = NetAddress::new(TEST_NETWORK, TEST_NETWORK_MASK, TEST_BROADCAST);
        let mut sensor = Sensor::new(home_net);

        let observation_id_1 = ObservationID::new();
        let observation_id_2 = ObservationID::new();

        sensor.add_observations_id(123, observation_id_1);
        sensor.add_observations_id(456, observation_id_2);

        assert!(sensor.get_observation_id(123).is_some());
        assert!(sensor.get_observation_id(456).is_some());
        assert!(sensor.get_observation_id(789).is_none());
    }
}
