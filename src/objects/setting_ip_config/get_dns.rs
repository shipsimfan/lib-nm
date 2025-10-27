use crate::NMSettingIPConfig;
use std::ffi::{c_char, c_int};

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `settings` - the [`NMSettingIPConfig`]
    ///  * `idx` - index number of the DNS server to return
    ///
    /// # Returns
    ///  the IP address of the DNS server at index `idx`
    pub fn nm_setting_ip_config_get_dns(
        settings: *mut NMSettingIPConfig,
        idx: c_int,
    ) -> *const c_char;
}
