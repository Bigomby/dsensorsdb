pub mod bindings;

use sensor::Sensor;

use std::collections::HashMap;

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

    pub fn list_sensors(&self) -> Vec<u32> {
        let mut sensor_ids = Vec::new();
        for id in self.database.keys() {
            sensor_ids.push(*id);
        }

        sensor_ids
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.database.insert(sensor.get_ip(), sensor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_sensors() {
        let mut database = SensorsDB::new();

        let sensor_1 = Sensor::new(1234);
        let sensor_2 = Sensor::new(5678);

        database.add_sensor(sensor_1);
        database.add_sensor(sensor_2);

        let sensor_in_db_1 = database.get_sensor(1234);
        let sensor_in_db_2 = database.get_sensor(5678);
        let sensor_in_db_3 = database.get_sensor(0000);

        assert!(sensor_in_db_1.is_some());
        assert!(sensor_in_db_2.is_some());
        assert!(sensor_in_db_3.is_none());
    }
}
