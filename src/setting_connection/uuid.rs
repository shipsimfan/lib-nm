use crate::{NMSettingConnection, raw::NM_SETTING_CONNECTION_UUID};
use std::ffi::{CStr, CString};

impl<'connection> NMSettingConnection<'connection> {
    /// Set the UUID of the connection
    pub fn set_uuid(&self, uuid: &str) {
        let uuid = CString::new(uuid).unwrap();
        self.set_uuid_raw(&uuid);
    }

    /// Set the UUID of the connection using a raw [`CStr`]
    pub fn set_uuid_raw(&self, uuid: &CStr) {
        unsafe { self.set(NM_SETTING_CONNECTION_UUID, uuid) };
    }
}
