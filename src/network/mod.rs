pub mod bindings;

use std::net::IpAddr;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct NetAddress {
    network_address: [u8; 16],
    network_mask: [u8; 16],
    broadcast: [u8; 16],
}

impl NetAddress {
    pub fn new(network_address: [u8; 16], network_mask: [u8; 16], broadcast: [u8; 16]) -> Self {
        NetAddress {
            network_address: network_address,
            network_mask: network_mask,
            broadcast: broadcast,
        }
    }
}

pub struct Network {
    name: String,
    addres_as_str: String,
    net_address: NetAddress,
}

impl Network {
    pub fn new(net_address: NetAddress, name: &str) -> Self {
        Network {
            name: String::from(name),
            net_address: net_address,
            addres_as_str: format!("{}", IpAddr::from(net_address.network_address)),
        }
    }

    pub fn get_address_str(&self) -> &str {
        &self.addres_as_str
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_net_address(&self) -> &[u8; 16] {
        &self.net_address.network_address
    }
}
