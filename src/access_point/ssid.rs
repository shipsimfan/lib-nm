use crate::{NMAccessPoint, raw::nm_access_point_get_ssid};
use glib_2::raw::glib::g_bytes_get_data;
use std::ptr::null_mut;

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Get the SSID of the access point
    pub fn ssid(&self) -> &[u8] {
        let bytes = unsafe { nm_access_point_get_ssid(self.handle) };
        if bytes == null_mut() {
            return &[];
        }

        let mut size = 0;
        let ptr = unsafe { g_bytes_get_data(bytes, &mut size) };
        if size == 0 {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(ptr.cast(), size) }
    }
}
