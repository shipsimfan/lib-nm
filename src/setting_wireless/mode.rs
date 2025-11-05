use crate::{NMSettingWireless, raw::NM_SETTING_WIRELESS_MODE};
use std::ffi::{CStr, CString};

impl<'connection> NMSettingWireless<'connection> {
    /// Set the mode of the wireless connection
    pub fn set_mode(&self, mode: &str) {
        let mode = CString::new(mode).unwrap();
        self.set_mode_raw(&mode);
    }

    /// Set the mode of the wireless connection using a raw [`CStr`]
    pub fn set_mode_raw(&self, mode: &CStr) {
        unsafe { self.set(NM_SETTING_WIRELESS_MODE, mode) };
    }
}
