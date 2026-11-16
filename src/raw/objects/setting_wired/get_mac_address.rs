use crate::raw::NMSettingWired;
use std::ffi::c_char;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingWired`]
    ///
    /// # Returns
    ///  the “mac-address” property of the setting
    pub fn nm_setting_wired_get_mac_address(setting: *mut NMSettingWired) -> *const c_char;
}
