use crate::raw::NMSetting;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSettingWirelessSecurity;

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMSettingWirelessSecurity`] object with default values.
    ///
    /// # Returns
    /// the new empty [`NMSettingWirelessSecurity`] object.
    pub fn nm_setting_wireless_security_new() -> *mut NMSetting;
}
