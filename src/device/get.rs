use crate::{NMClient, NMDevice, raw};

impl<'client> NMDevice<'client> {
    /// Get the [`NMClient`] this device belongs to
    pub fn client(&self) -> Option<&'client NMClient> {
        self.client
    }

    /// Get the underlying handle to the device
    pub const unsafe fn handle(&self) -> *mut raw::NMDevice {
        self.handle
    }
}
