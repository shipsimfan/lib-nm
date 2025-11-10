use crate::{
    NMSettingIPConfig,
    raw::{NM_SETTING_IP_CONFIG_GATEWAY, nm_setting_ip_config_get_gateway},
};
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    ptr::null_mut,
};

impl<'connection> NMSettingIPConfig<'connection> {
    /// Get the IP address of the gateway
    pub fn gateway(&self) -> Option<Cow<str>> {
        self.gateway_raw().map(CStr::to_string_lossy)
    }

    /// Get the IP address of the gateway as a raw [`CStr`]
    pub fn gateway_raw(&self) -> Option<&CStr> {
        let ptr = unsafe { nm_setting_ip_config_get_gateway(self.handle()) };
        if ptr == null_mut() {
            return None;
        }

        Some(unsafe { CStr::from_ptr(ptr) })
    }

    /// Sets the IP address of the gateway
    pub fn set_gateway(&self, gateway: &str) {
        let gateway = CString::new(gateway).unwrap();
        self.set_gateway_raw(&gateway);
    }

    /// Sets the IP address of the gateway as a raw [`CStr`]
    pub fn set_gateway_raw(&self, gateway: &CStr) {
        unsafe { self.set(NM_SETTING_IP_CONFIG_GATEWAY, gateway) };
    }
}
