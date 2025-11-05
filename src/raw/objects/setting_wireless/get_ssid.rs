use crate::raw::NMSettingWireless;
use glib_2::raw::glib::GBytes;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingWireless`]
    ///
    /// # Returns
    /// the "ssid" property of the setting.
    pub fn nm_setting_wireless_get_ssid(setting: *mut NMSettingWireless) -> *mut GBytes;
}
