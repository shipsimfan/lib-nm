use crate::raw::NMSettingWireless;
use std::ffi::c_char;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingWireless`]
    ///
    /// # Returns
    ///  the “mac-address” property of the setting
    pub fn nm_setting_wireless_get_mac_address(setting: *mut NMSettingWireless) -> *const c_char;
}
