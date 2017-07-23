use network::{Network, NetAddress};

use libc::c_char;

#[no_mangle]
pub extern "C" fn network_new(address: NetAddress, name: *const c_char) -> *mut Network {
    unimplemented!()
}
