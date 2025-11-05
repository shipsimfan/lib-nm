use crate::{NMSettingConnection, raw::NM_SETTING_CONNECTION_TYPE};
use std::ffi::{CStr, CString};

impl<'connection> NMSettingConnection<'connection> {
    /// Set the type of the connection
    pub fn set_type(&self, r#type: &str) {
        let r#type = CString::new(r#type).unwrap();
        self.set_type_raw(&r#type);
    }

    /// Set the type of the connection using a raw [`CStr`]
    pub fn set_type_raw(&self, r#type: &CStr) {
        unsafe { self.set(NM_SETTING_CONNECTION_TYPE, r#type) };
    }
}
