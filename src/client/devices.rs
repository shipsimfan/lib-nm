use crate::{NMClient, NMDeviceIter, raw::nm_client_get_devices};

impl NMClient {
    /// Get an iterator over the devices presented through NetworkManager
    pub fn devices<'client>(&'client self) -> NMDeviceIter<'client> {
        let handle = unsafe { nm_client_get_devices(self.handle()) };
        unsafe { NMDeviceIter::new(handle, self) }
    }
}
