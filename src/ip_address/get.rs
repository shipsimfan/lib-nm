use crate::{NMIPAddress, NMSetting, raw};

impl<'setting, 'connection> NMIPAddress<'setting, 'connection> {
    /// Get the [`NMSetting`] this IP address belongs to
    pub fn setting(&self) -> Option<&'setting NMSetting<'connection>> {
        self.setting
    }

    /// Get the underlying handle to the IP address
    pub const unsafe fn handle(&self) -> *mut raw::NMIPAddress {
        self.handle
    }
}
