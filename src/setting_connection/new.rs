use crate::{
    NMConnection, NMSetting, NMSettingConnection,
    raw::{self, nm_setting_connection_new},
};

impl<'connection> NMSettingConnection<'connection> {
    /// Creates a new [`NMSettingConnection`] object with default values
    pub fn new() -> Self {
        let handle = unsafe { nm_setting_connection_new() };
        unsafe { NMSettingConnection::new_raw(handle, None) }
    }

    /// Create a new [`NMSettingConnection`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMSettingConnection,
        connection: Option<&'connection NMConnection>,
    ) -> Self {
        let settings = unsafe { NMSetting::new_raw(handle, connection) };
        NMSettingConnection { settings }
    }
}
