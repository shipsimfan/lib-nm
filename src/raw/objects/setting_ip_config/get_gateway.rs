use crate::NMSettingIPConfig;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingIPConfig`]
    ///
    /// # Return Value
    /// the IP address of the gateway associated with this configuration, or [`null`].
    pub fn nm_setting_ip_config_get_gateway(setting: *mut NMSettingIPConfig) -> *const c_char;
}
