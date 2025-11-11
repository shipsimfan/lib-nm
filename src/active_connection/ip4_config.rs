use crate::{NMActiveConnection, NMIPConfig, raw::nm_active_connection_get_ip4_config};
use std::ptr::null_mut;

impl<'owner, Owner> NMActiveConnection<'owner, Owner> {
    /// Gets the current IPv4 [`NMIPConfig`] associated with the [`NMActiveConnection`]
    pub fn ip4_config(&self) -> Option<NMIPConfig<Self>> {
        let handle = unsafe { nm_active_connection_get_ip4_config(self.handle()) };
        if handle == null_mut() {
            return None;
        }

        Some(unsafe { NMIPConfig::new_raw(handle, Some(self), false) })
    }
}
