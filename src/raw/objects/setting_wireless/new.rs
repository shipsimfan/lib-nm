use crate::raw::NMSetting;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSettingWireless;

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMSettingWireless`] object with default values.
    ///
    /// # Returns
    /// the new empty [`NMSettingWireless`] object.
    pub fn nm_setting_wireless_new() -> *mut NMSetting;
}
