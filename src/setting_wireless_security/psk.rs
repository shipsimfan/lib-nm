use crate::{NMSettingWirelessSecurity, raw::NM_SETTING_WIRELESS_SECURITY_PSK};
use std::ffi::{CStr, CString};

impl<'connection> NMSettingWirelessSecurity<'connection> {
    /// Set the pre-shared key of for wireless connections
    pub fn set_psk(&self, psk: &str) {
        let psk = CString::new(psk).unwrap();
        self.set_psk_raw(&psk);
    }

    /// Set the pre-shared key of for wireless connections using a raw [`CStr`]
    pub fn set_psk_raw(&self, psk: &CStr) {
        unsafe { self.set(NM_SETTING_WIRELESS_SECURITY_PSK, psk) };
    }
}
