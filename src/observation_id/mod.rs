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
    applications: HashMap<u32, Application>,
    networks: HashMap<IpAddr, Network>,
    templates: HashMap<u16, *mut c_void>,
    want_client_dns: bool,
    want_target_dns: bool,
    ptr_dns_target: bool,
    exporter_in_wan_side: bool,
}

impl ObservationID {
    pub fn new(id: u32) -> ObservationID {
        ObservationID {
            id: id,
            enrichment: None,
            fallback_first_switch: None,
            applications: HashMap::new(),
            networks: HashMap::new(),
            templates: HashMap::new(),
            want_client_dns: false,
            want_target_dns: false,
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

    pub fn list_templates(&self) -> Vec<u16> {
        let mut templates = Vec::new();
        for template in self.templates.keys() {
            templates.push(*template);
        }

        templates
    }

    pub fn get_template(&self, id: u16) -> Option<&*mut c_void> {
        match self.templates.get(&id) {
            Some(template) => Some(template),
            None => None,
        }
    }

    pub fn get_enrichment(&self) -> Option<&[u8]> {
        match self.enrichment {
            Some(ref enrichment) => Some(enrichment),
            None => None,
        }
    }

    pub fn get_application(&self, id: u32) -> Option<&Application> {
        self.applications.get(&id)
    }

    pub fn get_fallback_first_switch(&self) -> Option<i64> {
        self.fallback_first_switch
    }

    pub fn want_client_dns(&self) -> bool {
        self.want_client_dns
    }

    pub fn want_target_dns(&self) -> bool {
        self.want_target_dns
    }

    pub fn add_template(&mut self, id: u16, template: *mut c_void) {
        self.templates.insert(id, template);
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

    #[repr(C)]
    #[derive(Debug, PartialEq)]
    struct Template {
        example_data_1: [u8; 16],
        example_data_2: [u32; 4],
        example_data_3: String,
    }

    #[test]
    fn test_add_template() {
        let template = Box::new(Template {
            example_data_1: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            example_data_2: [10, 20, 30, 50],
            example_data_3: String::from("Hello world"),
        });

        let mut observation_id = ObservationID::new(1234);
        let raw_data = Box::into_raw(template) as *mut c_void;

        observation_id.add_template(42, raw_data);
        let template_ref = observation_id.get_template(42);

        let tmpl = *template_ref.unwrap() as *mut Template;

        unsafe {
            assert_eq!(
                (*tmpl).example_data_1,
                [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
            );
            assert_eq!((*tmpl).example_data_2, [10, 20, 30, 50]);
            assert_eq!((*tmpl).example_data_3, "Hello world");
        }
    }
}
