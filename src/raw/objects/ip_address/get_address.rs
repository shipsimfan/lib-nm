use crate::raw::NMIPAddress;
use std::ffi::c_char;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the IP address property of this address object.
    ///
    /// # Parameters
    ///  * `address` - the [`NMIPAddress`]
    ///
    /// # Returns
    ///  the IP address
    pub fn nm_ip_address_get_address(address: *mut NMIPAddress) -> *const c_char;
}
