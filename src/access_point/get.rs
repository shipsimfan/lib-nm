use crate::{NMAccessPoint, NMDevice, raw};

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Get the [`NMDevice`] this access point belongs to
    pub fn device(&self) -> &'device NMDevice {
        self.device
    }

    /// Get the underlying handle to the access point
    pub const unsafe fn handle(&self) -> *mut raw::NMAccessPoint {
        self.handle
    }
}
