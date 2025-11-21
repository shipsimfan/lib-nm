use crate::raw::NMSettingIPConfig;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMSettingIP4Config, NMSettingIP6Config};

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingIPConfig`]
    ///
    /// # Returns
    ///  the “method” property of the setting; see [`NMSettingIP4Config`] and
    /// [`NMSettingIP6Config`] for details of the methods available with each type.
    pub fn nm_setting_ip_config_get_method(setting: *mut NMSettingIPConfig) -> *const c_char;
}
