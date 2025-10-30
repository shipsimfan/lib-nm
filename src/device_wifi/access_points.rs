use crate::{NMAccessPointIter, NMDeviceWifi, raw::nm_device_wifi_get_access_points};

impl<'client> NMDeviceWifi<'client> {
    /// Gets all the scanned access points of the [`NMDeviceWifi`]
    pub fn access_points<'device>(&'device self) -> NMAccessPointIter<'client, 'device> {
        let ptr_array = unsafe { &*nm_device_wifi_get_access_points(self.handle()) };

        let slice = if ptr_array.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(ptr_array.data, ptr_array.len as _) }
        };

        unsafe { NMAccessPointIter::new(slice.iter(), self) }
    }
}
