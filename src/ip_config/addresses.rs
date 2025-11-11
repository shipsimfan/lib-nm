use crate::{NMIPAddressIter, NMIPConfig, raw::nm_ip_config_get_addresses};

impl<'owner, Owner> NMIPConfig<'owner, Owner> {
    /// Gets the IP addresses (containing the address, prefix, and gateway)
    pub fn addresses(&self) -> NMIPAddressIter<Self> {
        let handle = unsafe { nm_ip_config_get_addresses(self.handle()) };
        unsafe { NMIPAddressIter::new(handle, self, false) }
    }
}
