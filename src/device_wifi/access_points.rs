use crate::{NMAccessPointIter, NMDeviceWifi, raw::nm_device_wifi_get_access_points};

impl<'client> NMDeviceWifi<'client> {
    /// Gets all the scanned access points of the [`NMDeviceWifi`]
    pub fn access_points<'device>(&'device self) -> NMAccessPointIter<'client, 'device> {
        let handle = unsafe { nm_device_wifi_get_access_points(self.handle()) };
        unsafe { NMAccessPointIter::new(handle, self) }
    }
}
