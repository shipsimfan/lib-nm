use crate::raw::NMIPAddress;
use glib_2::glib::guint;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the IP address prefix (ie "24" or "30" etc) property of this address object.
    ///
    /// # Parameters
    ///  * `address` - the [`NMIPAddress`]
    ///
    /// # Returns
    /// the IP address prefix
    pub fn nm_ip_address_get_prefix(address: *mut NMIPAddress) -> guint;
}
