use crate::{NMSettingConnection, raw::NM_SETTING_CONNECTION_ID};
use std::ffi::{CStr, CString};

impl<'connection> NMSettingConnection<'connection> {
    /// Set the ID of the connection
    pub fn set_id(&self, id: &str) {
        let id = CString::new(id).unwrap();
        self.set_id_raw(&id);
    }

    /// Set the ID of the connection using a raw [`CStr`]
    pub fn set_id_raw(&self, id: &CStr) {
        unsafe { self.set(NM_SETTING_CONNECTION_ID, id) };
    }
}
