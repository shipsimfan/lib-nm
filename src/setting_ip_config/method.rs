use crate::{NMSettingIPConfig, raw::NM_SETTING_IP_CONFIG_METHOD};
use std::ffi::{CStr, CString};

impl<'connection> NMSettingIPConfig<'connection> {
    /// Set the IP method of the connection
    pub fn set_method(&self, method: &str) {
        let method = CString::new(method).unwrap();
        self.set_method_raw(&method);
    }

    /// Set the IP method of the connection using a raw [`CStr`]
    pub fn set_method_raw(&self, method: &CStr) {
        unsafe { self.set(NM_SETTING_IP_CONFIG_METHOD, method) };
    }
}
