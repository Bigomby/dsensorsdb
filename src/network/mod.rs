pub mod bindings;

pub type IPAddress = [u8; 16];

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct NetAddress {
    network_address: IPAddress,
    network_mask: IPAddress,
    broadcast: IPAddress,
}

impl NetAddress {
    pub fn new(network_address: IPAddress, network_mask: IPAddress, broadcast: IPAddress) -> Self {
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
