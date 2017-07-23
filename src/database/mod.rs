pub mod bindings;

use sensor::Sensor;

use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Default)]
pub struct SensorsDB {
    database: HashMap<IpAddr, Sensor>,
}

impl SensorsDB {
    pub fn new() -> Self {
        SensorsDB::default()
    }

    pub fn get_sensor(&self, ip: IpAddr) -> Option<&Sensor> {
        self.database.get(&ip)
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.database.insert(sensor.get_ip(), sensor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv6Addr};

    const TEST_NETWORK_1: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 168, 1, 1];
    const TEST_NETWORK_2: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 168, 1, 2];

    #[test]
    fn test_add_sensors() {
        let mut database = SensorsDB::new();

        let sensor_1 = Sensor::new(IpAddr::from(TEST_NETWORK_1));
        let sensor_2 = Sensor::new(IpAddr::from(TEST_NETWORK_2));

        database.add_sensor(sensor_1);
        database.add_sensor(sensor_2);

        let ip_str_1: Ipv6Addr = "::192.168.1.1".parse().unwrap();
        let ip_str_2: Ipv6Addr = "::192.168.1.2".parse().unwrap();
        let ip_str_3: Ipv6Addr = "::192.168.1.3".parse().unwrap();

        let sensor_in_db_1 = database.get_sensor(IpAddr::from(Ipv6Addr::from(ip_str_1)));
        let sensor_in_db_2 = database.get_sensor(IpAddr::from(Ipv6Addr::from(ip_str_2)));
        let sensor_in_db_3 = database.get_sensor(IpAddr::from(Ipv6Addr::from(ip_str_3)));

        assert!(sensor_in_db_1.is_some());
        assert!(sensor_in_db_2.is_some());
        assert!(sensor_in_db_3.is_none());
    }
}
