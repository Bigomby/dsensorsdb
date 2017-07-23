pub mod bindings;

use std::collections::HashMap;

use sensor::Sensor;

#[derive(Default)]
pub struct SensorsDB {
    database: HashMap<u32, Sensor>,
}

impl SensorsDB {
    pub fn new() -> Self {
        SensorsDB::default()
    }

    pub fn get_sensor(&self, ip: u32) -> Option<&Sensor> {
        self.database.get(&ip)
    }

    pub fn add_sensor(&mut self, ip: u32, sensor: Sensor) {
        self.database.insert(ip, sensor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::network::{IPAddress, NetAddress};

    const TEST_NETWORK: IPAddress = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
                                     0x01, 0x00, 0x01, 0x02, 0x03, 0x04];
    const TEST_NETWORK_MASK: IPAddress = [0x0, 0x0, 0x0, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00,
                                          0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    const TEST_BROADCAST: IPAddress = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    #[test]
    fn test_database() {
        let mut database = SensorsDB::new();

        let home_net = NetAddress::new(TEST_NETWORK, TEST_NETWORK_MASK, TEST_BROADCAST);
        let sensor_1 = Sensor::new(home_net);
        let sensor_2 = Sensor::new(home_net);

        database.add_sensor(123, sensor_1);
        database.add_sensor(456, sensor_2);

        let sensor_in_db_1 = database.get_sensor(123);
        let sensor_in_db_2 = database.get_sensor(456);
        let sensor_in_db_3 = database.get_sensor(789);

        assert_eq!(home_net, sensor_in_db_1.unwrap().get_address());
        assert_eq!(home_net, sensor_in_db_2.unwrap().get_address());
        assert!(sensor_in_db_3.is_none());
    }
}
