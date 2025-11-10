use glib_2::raw::glib::FALSE;

use crate::{
    NMSettingIPConfig,
    raw::{
        nm_setting_ip_config_add_dns, nm_setting_ip_config_get_dns,
        nm_setting_ip_config_get_num_dns,
    },
};
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    ptr::null_mut,
};

impl<'connection> NMSettingIPConfig<'connection> {
    /// Get the number of configured DNS servers
    pub fn num_dns(&self) -> glib_2::raw::glib::guint {
        unsafe { nm_setting_ip_config_get_num_dns(self.handle()) }
    }

    /// Get the DNS server at index `idx`
    pub fn dns(&self, idx: glib_2::raw::glib::guint) -> Option<Cow<str>> {
        self.dns_raw(idx).map(CStr::to_string_lossy)
    }

    /// Get the DNS server at index `idx` as a raw [`CStr`]
    pub fn dns_raw(&self, idx: glib_2::raw::glib::guint) -> Option<&CStr> {
        let ptr = unsafe { nm_setting_ip_config_get_dns(self.handle(), idx as _) };
        if ptr == null_mut() {
            return None;
        }

        Some(unsafe { CStr::from_ptr(ptr) })
    }

    /// Adds a new DNS server to the setting
    pub fn add_dns(&self, dns: &str) -> bool {
        let dns = CString::new(dns).unwrap();
        self.add_dns_raw(&dns)
    }

    /// Adds a new DNS server to the setting as a raw [`CStr`]
    pub fn add_dns_raw(&self, dns: &CStr) -> bool {
        let result = unsafe { nm_setting_ip_config_add_dns(self.handle(), dns.as_ptr()) };
        result != FALSE
    }
}
