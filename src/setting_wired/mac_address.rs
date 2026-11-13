use crate::{NMSettingWired, raw::nm_setting_wired_get_mac_address};
use std::{borrow::Cow, ffi::CStr, ptr::null_mut};

impl<'connection> NMSettingWired<'connection> {
    /// Get the MAC address of the wired interface
    pub fn mac_address<'a>(&'a self) -> Option<Cow<'a, str>> {
        self.mac_address_raw().map(CStr::to_string_lossy)
    }

    /// Get the MAC address of the wired interface as a raw [`CStr`]
    pub fn mac_address_raw(&self) -> Option<&CStr> {
        let ptr = unsafe { nm_setting_wired_get_mac_address(self.handle()) };
        if ptr == null_mut() {
            return None;
        }

        Some(unsafe { CStr::from_ptr(ptr) })
    }
}
