use crate::{NMConnection, raw::nm_connection_get_id};
use std::{borrow::Cow, ffi::CStr};

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSettingConnection;

impl NMConnection {
    /// A shortcut to return the ID from the connection's [`NMSettingConnection`]
    pub fn id(&self) -> Cow<str> {
        self.id_raw().to_string_lossy()
    }

    /// A shortcut to return the ID from the connection's [`NMSettingConnection`] as a raw [`CStr`]
    pub fn id_raw(&self) -> &CStr {
        unsafe { CStr::from_ptr(nm_connection_get_id(self.handle())) }
    }
}
