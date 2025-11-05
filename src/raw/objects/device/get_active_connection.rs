use crate::raw::{NMActiveConnection, NMDevice};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the [`NMActiveConnection`] object which owns this device during activation.
    ///
    /// # Parameters
    ///  * `device` - a [`NMDevice`]
    ///
    /// # Returns
    /// the [`NMActiveConnection`] or [`null_mut`] if the device is not part of an active
    /// connection.
    pub fn nm_device_get_active_connection(device: *mut NMDevice) -> *mut NMActiveConnection;
}
