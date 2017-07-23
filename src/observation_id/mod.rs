pub mod bindings;

use application::Application;
use network::Network;
use libc::c_void;

use std::collections::HashMap;
use std::net::IpAddr;

pub struct ObservationID {
    id: u32,
    enrichment: Option<Vec<u8>>,
    fallback_first_switch: Option<i64>,
    observation_domain_flags: Option<u8>,
    applications: HashMap<u32, Application>,
    networks: HashMap<IpAddr, Network>,
    templates: HashMap<u32, *const c_void>,
    ptr_dns_target: bool,
    exporter_in_wan_side: bool,
}

impl ObservationID {
    pub fn new(id: u32) -> ObservationID {
        ObservationID {
            id: id,
            enrichment: None,
            fallback_first_switch: None,
            observation_domain_flags: None,
            applications: HashMap::new(),
            networks: HashMap::new(),
            templates: HashMap::new(),
            ptr_dns_target: false,
            exporter_in_wan_side: false,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_network(&self, ip: IpAddr) -> Option<&Network> {
        self.networks.get(&ip)
    }

    pub fn set_enrichment(&mut self, enrichment: &[u8]) {
        self.enrichment = Some(Vec::from(enrichment));
    }

    pub fn set_fallback_first_switch(&mut self, fallback_first_switch: i64) {
        self.fallback_first_switch = Some(fallback_first_switch);
    }

    pub fn set_exporter_in_wan_side(&mut self) {
        self.exporter_in_wan_side = true;
    }

    pub fn enable_ptr_dns_target(&mut self) {
        self.ptr_dns_target = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn add_applications() {
    //     let observation_id = ObservationID::new(TEST_ID);
    //     assert_eq!(observation_id.get_id(), TEST_ID);
    // }
}
