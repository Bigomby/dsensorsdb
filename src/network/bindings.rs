use network::{Network, NetAddress};

use libc::c_char;

#[no_mangle]
pub extern "C" fn network_new(address: NetAddress, name: *const c_char) -> *mut Network {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn network_get_name(network_ptr: *const Network) -> *const c_char {
    assert!(!network_ptr.is_null());
    let network = unsafe { &*network_ptr };

    network.get_name().as_ptr() as *const c_char
}
