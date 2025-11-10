use crate::{
    NMSettingWirelessSecurity,
    raw::{NM_SETTING_WIRELESS_SECURITY_PSK, nm_setting_wireless_security_get_psk},
};
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    ptr::null_mut,
};

impl<'connection> NMSettingWirelessSecurity<'connection> {
    /// Get the pre-shared key for wireless connections
    pub fn psk(&self) -> Option<Cow<str>> {
        self.psk_raw().map(CStr::to_string_lossy)
    }

    /// Get the pre-shared key for wireless connections using a raw [`CStr`]
    pub fn psk_raw(&self) -> Option<&CStr> {
        let ptr = unsafe { nm_setting_wireless_security_get_psk(self.handle()) };
        if ptr == null_mut() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    /// Set the pre-shared key for wireless connections
    pub fn set_psk(&self, psk: &str) {
        let psk = CString::new(psk).unwrap();
        self.set_psk_raw(&psk);
    }

    /// Set the pre-shared key for wireless connections using a raw [`CStr`]
    pub fn set_psk_raw(&self, psk: &CStr) {
        unsafe { self.set(NM_SETTING_WIRELESS_SECURITY_PSK, psk) };
    }
}
