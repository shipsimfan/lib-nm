use crate::{
    NMSettingIPConfig,
    raw::{NM_SETTING_IP_CONFIG_METHOD, nm_setting_ip_config_get_method},
};
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
};

impl<'connection> NMSettingIPConfig<'connection> {
    /// Get the method used to get and set the IP config
    pub fn method(&self) -> Option<Cow<str>> {
        self.method_raw().map(CStr::to_string_lossy)
    }

    /// Get the method used to get and set the IP config as a raw [`CStr`]
    pub fn method_raw(&self) -> Option<&CStr> {
        let ptr = unsafe { nm_setting_ip_config_get_method(self.handle()) };
        Some(unsafe { CStr::from_ptr(ptr) })
    }

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
