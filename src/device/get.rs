use crate::{NMClient, NMDevice, raw};

impl<'client> NMDevice<'client> {
    /// Get the [`NMClient`] this device belongs to
    pub fn client(&self) -> &'client NMClient {
        self.client
    }

    /// Get the underlying handle to the device
    pub unsafe fn handle(&self) -> *mut raw::NMDevice {
        self.handle
    }
}
