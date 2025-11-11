use crate::{NMActiveConnection, NMDevice, raw::nm_device_get_active_connection};
use std::ptr::null_mut;

impl<'client> NMDevice<'client> {
    /// Gets the [`NMActiveConnection`] object which owns this device during activation
    pub fn active_connection(&self) -> Option<NMActiveConnection<Self>> {
        let handle = unsafe { nm_device_get_active_connection(self.handle()) };
        if handle == null_mut() {
            return None;
        }

        Some(unsafe { NMActiveConnection::new_raw(handle, Some(self), false) })
    }
}
