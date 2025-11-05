use crate::{NMConnection, NMSettingConnection, raw::nm_connection_get_setting_connection};
use std::ptr::null_mut;

impl NMConnection {
    /// A shortcut to return any [`NMSettingConnection`] the connection might contain.
    pub fn setting_connection(&self) -> Option<NMSettingConnection> {
        let handle = unsafe { nm_connection_get_setting_connection(self.handle()) };
        if handle == null_mut() {
            return None;
        }
        Some(unsafe { NMSettingConnection::new_raw(handle, Some(self)) })
    }
}
