use crate::{NMSettingWirelessSecurity, raw::NM_SETTING_WIRELESS_SECURITY_KEY_MGMT};
use std::ffi::{CStr, CString};

impl<'connection> NMSettingWirelessSecurity<'connection> {
    /// Set the key management for wireless connections
    pub fn set_key_mgmt(&self, key_mgmt: &str) {
        let key_mgmt = CString::new(key_mgmt).unwrap();
        self.set_key_mgmt_raw(&key_mgmt);
    }

    /// Set the key management for wireless connections using a raw [`CStr`]
    pub fn set_key_mgmt_raw(&self, key_mgmt: &CStr) {
        unsafe { self.set(NM_SETTING_WIRELESS_SECURITY_KEY_MGMT, key_mgmt) };
    }
}
