use crate::raw::NMIPConfig;
use glib_2::raw::glib::GPtrArray;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMIPAddress;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the IP addresses (containing the address, prefix, and gateway).
    ///
    /// # Parameters
    ///  * `config` - a [`NMIPConfig`]
    ///
    /// # Returns
    /// the [`GPtrArray`] containing [`NMIPAddress`]es. This is the internal copy used by the
    /// configuration and must not be modified. The library never modifies the returned array and
    /// thus it is safe for callers to reference and keep using it.
    pub fn nm_ip_config_get_addresses(config: *mut NMIPConfig) -> *mut GPtrArray;
}
