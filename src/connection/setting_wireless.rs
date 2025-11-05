use crate::{NMConnection, NMSettingWireless, raw::nm_connection_get_setting_wireless};
use std::ptr::null_mut;

impl NMConnection {
    /// A shortcut to return any [`NMSettingWireless`] the connection might contain
    pub fn setting_wireless(&self) -> Option<NMSettingWireless> {
        let handle = unsafe { nm_connection_get_setting_wireless(self.handle()) };
        if handle == null_mut() {
            return None;
        }
        Some(unsafe { NMSettingWireless::new_raw(handle, Some(self)) })
    }
}
