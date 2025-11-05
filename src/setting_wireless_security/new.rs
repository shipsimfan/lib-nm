use crate::{
    NMConnection, NMSetting, NMSettingWirelessSecurity,
    raw::{self, nm_setting_wireless_security_new},
};

impl<'connection> NMSettingWirelessSecurity<'connection> {
    /// Creates a new [`NMSettingWirelessSecurity`] object with default values
    pub fn new() -> Self {
        let handle = unsafe { nm_setting_wireless_security_new() };
        unsafe { NMSettingWirelessSecurity::new_raw(handle, None) }
    }

    /// Create a new [`NMSettingWirelessSecurity`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMSettingWirelessSecurity,
        connection: Option<&'connection NMConnection>,
    ) -> Self {
        let settings = unsafe { NMSetting::new_raw(handle, connection) };
        NMSettingWirelessSecurity { settings }
    }
}
