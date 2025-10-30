use crate::NMSetting;

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSettingIP4Config;

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMSettingIP4Config`] object with default values.
    ///
    /// # Returns
    /// the new empty [`NMSettingIP4Config`] object.
    pub fn nm_setting_ip4_config_new() -> *mut NMSetting;
}
