use crate::{NMConnection, NMSettingIP4Config, raw::nm_connection_get_setting_ip4_config};
use std::ptr::null_mut;

impl NMConnection {
    /// A shortcut to return any [`NMSettingIP4Config`] the connection might contain.
    pub fn setting_ip4_config(&self) -> Option<NMSettingIP4Config> {
        let handle = unsafe { nm_connection_get_setting_ip4_config(self.handle()) };
        if handle == null_mut() {
            return None;
        }
        Some(unsafe { NMSettingIP4Config::new_raw(handle, Some(self)) })
    }
}
