use crate::raw::NMSettingWirelessSecurity;
use std::ffi::c_char;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingWirelessSecurity`]
    ///
    /// # Returns
    ///  the “psk” property of the setting
    pub fn nm_setting_wireless_security_get_psk(
        setting: *mut NMSettingWirelessSecurity,
    ) -> *const c_char;
}
