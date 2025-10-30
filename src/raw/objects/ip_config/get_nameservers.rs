use crate::NMIPConfig;
use std::ffi::c_char;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the domain name servers (DNS).
    ///
    /// # Parameters
    ///  * `config` - a [`NMIPConfig`]
    ///
    /// # Returns
    /// the array of nameserver IP addresses.
    pub fn nm_ip_config_get_nameservers(config: *mut NMIPConfig) -> *const *const c_char;
}
