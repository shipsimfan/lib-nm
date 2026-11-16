use crate::{NMIPAddress, raw};

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSetting;

impl<'owner, Owner> NMIPAddress<'owner, Owner> {
    /// Get the [`NMSetting`] this IP address belongs to
    pub fn setting(&self) -> Option<&'owner Owner> {
        self.setting
    }

    /// Get the underlying handle to the IP address
    pub const unsafe fn handle(&self) -> *mut raw::NMIPAddress {
        self.handle
    }
}
