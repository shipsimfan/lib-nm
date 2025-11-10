use crate::{NMIPAddress, raw::nm_ip_address_get_prefix};

impl<'setting, 'connection> NMIPAddress<'setting, 'connection> {
    /// Gets the IP address prefix (ie "24" or "30" etc) property of this address object.
    pub fn prefix(&self) -> glib_2::raw::glib::guint {
        unsafe { nm_ip_address_get_prefix(self.handle) }
    }
}
