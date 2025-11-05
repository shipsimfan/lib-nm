use crate::{NMSettingWireless, raw::NM_SETTING_WIRELESS_SSID};
use glib_2::glib::GBytes;

impl<'connection> NMSettingWireless<'connection> {
    /// Set the SSID of the wireless connection
    pub fn set_ssid(&self, ssid: &GBytes) {
        unsafe { self.set(NM_SETTING_WIRELESS_SSID, ssid) };
    }
}
