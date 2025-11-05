use crate::raw::NMSetting;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSettingIP6Config;

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMSettingIP6Config`] object with default values.
    ///
    /// # Returns
    /// the new empty [`NMSettingIP6Config`] object.
    pub fn nm_setting_ip6_config_new() -> *mut NMSetting;
}
