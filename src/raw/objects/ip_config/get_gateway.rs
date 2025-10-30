use crate::raw::NMIPConfig;
use std::ffi::c_char;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the IP gateway address.
    ///
    /// # Parameters
    ///  * `config` - a [`NMIPConfig`]
    ///
    /// # Returns
    /// the IP address of the gateway.
    pub fn nm_ip_config_get_gateway(config: *mut NMIPConfig) -> *const c_char;
}
