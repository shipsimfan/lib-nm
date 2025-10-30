use crate::{NMClient, NMDeviceIter, raw::nm_client_get_devices};

impl NMClient {
    /// Get an iterator over the devices presented through NetworkManager
    pub fn devices(&self) -> NMDeviceIter {
        let ptr_array = unsafe { &*nm_client_get_devices(self.handle) };

        let slice = if ptr_array.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(ptr_array.data, ptr_array.len as _) }
        };

        unsafe { NMDeviceIter::new(slice.iter(), self) }
    }
}
