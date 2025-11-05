use crate::{
    NMConnection, NMSetting, NMSettingWireless,
    raw::{self, nm_setting_wireless_new},
};

impl<'connection> NMSettingWireless<'connection> {
    /// Creates a new [`NMSettingWireless`] object with default values.
    pub fn new() -> Self {
        let handle = unsafe { nm_setting_wireless_new() };
        unsafe { NMSettingWireless::new_raw(handle, None) }
    }

    /// Create a new [`NMSettingWireless`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMSettingWireless,
        connection: Option<&'connection NMConnection>,
    ) -> Self {
        let settings = unsafe { NMSetting::new_raw(handle, connection) };
        NMSettingWireless { settings }
    }
}
