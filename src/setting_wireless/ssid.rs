use crate::{
    NMSettingWireless,
    raw::{NM_SETTING_WIRELESS_SSID, nm_setting_wireless_get_ssid},
};
use glib_2::glib::GBytes;

impl<'connection> NMSettingWireless<'connection> {
    /// Get the SSID of the wireless connection
    pub fn ssid(&self) -> &[u8] {
        let handle = unsafe { nm_setting_wireless_get_ssid(self.handle()) };
        let bytes = unsafe { GBytes::new_raw(handle, false) };
        bytes.data()
    }

    /// Set the SSID of the wireless connection
    pub fn set_ssid(&self, ssid: &GBytes) {
        unsafe { self.set(NM_SETTING_WIRELESS_SSID, ssid) };
    }
}
