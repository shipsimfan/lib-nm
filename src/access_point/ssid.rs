use crate::{NMAccessPoint, raw::nm_access_point_get_ssid};
use glib_2::glib::GBytes;
use std::ptr::null_mut;

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Get the SSID of the access point
    pub fn ssid(&self) -> Option<GBytes> {
        let handle = unsafe { nm_access_point_get_ssid(self.handle) };
        if handle == null_mut() {
            return None;
        }

        Some(unsafe { GBytes::new_raw(handle, false) })
    }
}
