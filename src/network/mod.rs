pub mod bindings;

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
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
