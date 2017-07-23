pub mod bindings;

use application::Application;
use network::{IPAddress, Network};
use libc::c_void;

use std::collections::HashMap;

pub struct ObservationID {
    enrichment: Option<Vec<u8>>,
    fallback_first_switch: Option<i64>,
    observation_domain_flags: Option<u8>,
    applications: HashMap<u32, Application>,
    networks: HashMap<IPAddress, Network>,
    templates: HashMap<u32, *const c_void>,
}

impl ObservationID {
    pub fn new() -> ObservationID {
        ObservationID {
            enrichment: None,
            fallback_first_switch: None,
            observation_domain_flags: None,
            applications: HashMap::new(),
            networks: HashMap::new(),
            templates: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn add_applications() {
    //     let observation_id = ObservationID::new(TEST_ID);
    //
    //     assert_eq!(observation_id.get_id(), TEST_ID);
    // }
}
