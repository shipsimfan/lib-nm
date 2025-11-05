use crate::{
    NMConnection, NMSettingWirelessSecurity, raw::nm_connection_get_setting_wireless_security,
};
use std::ptr::null_mut;

impl NMConnection {
    /// A shortcut to return any [`NMSettingWirelessSecurity`] the connection might contain
    pub fn setting_wireless_security(&self) -> Option<NMSettingWirelessSecurity> {
        let handle = unsafe { nm_connection_get_setting_wireless_security(self.handle()) };
        if handle == null_mut() {
            return None;
        }
        Some(unsafe { NMSettingWirelessSecurity::new_raw(handle, Some(self)) })
    }
}
