use crate::{NMClient, NMDeviceIter, raw::nm_client_get_devices};

impl NMClient {
    /// Get an iterator over the devices presented through NetworkManager
    pub fn devices(&self) -> NMDeviceIter {
        let handle = unsafe { nm_client_get_devices(self.handle()) };
        unsafe { NMDeviceIter::new(handle, self) }
    }
}
