use crate::{NMConnection, NMSettingWired, raw::nm_connection_get_setting_wired};
use std::ptr::null_mut;

impl NMConnection {
    /// A shortcut to return any [`NMSettingWired`] the connection might contain
    pub fn setting_wired<'connection>(&'connection self) -> Option<NMSettingWired<'connection>> {
        let handle = unsafe { nm_connection_get_setting_wired(self.handle()) };
        if handle == null_mut() {
            return None;
        }
        Some(unsafe { NMSettingWired::new_raw(handle, Some(self)) })
    }
}
