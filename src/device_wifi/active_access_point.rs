use crate::{NMAccessPoint, NMDeviceWifi, raw::nm_device_wifi_get_active_access_point};
use std::ptr::null_mut;

impl<'client> NMDeviceWifi<'client> {
    /// Gets the active [`NMAccessPoint`]
    pub fn active_access_point<'device>(&'device self) -> Option<NMAccessPoint<'client, 'device>> {
        let handle = unsafe { nm_device_wifi_get_active_access_point(self.handle()) };
        if handle == null_mut() {
            return None;
        }
        Some(unsafe { NMAccessPoint::new_raw(handle, self) })
    }
}
