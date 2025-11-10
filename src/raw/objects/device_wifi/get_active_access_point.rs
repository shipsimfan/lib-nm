use crate::raw::{NMAccessPoint, NMDeviceWifi};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the active [`NMAccessPoint`].
    ///
    /// # Parameters
    ///  * `device` - a [`NMDeviceWifi`]
    ///
    /// # Returns
    /// the access point or [`null_mut`] if none is active.
    pub fn nm_device_wifi_get_active_access_point(device: *mut NMDeviceWifi) -> *mut NMAccessPoint;
}
