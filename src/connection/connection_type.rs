use crate::{NMConnection, raw::nm_connection_get_connection_type};
use std::{borrow::Cow, ffi::CStr};

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSettingConnection;

impl NMConnection {
    /// A shortcut to return the type from the connection's [`NMSettingConnection`]
    pub fn connection_type(&self) -> Cow<str> {
        self.connection_type_raw().to_string_lossy()
    }

    /// A shortcut to return the type from the connection's [`NMSettingConnection`] as a raw
    /// [`CStr`]
    pub fn connection_type_raw(&self) -> &CStr {
        unsafe { CStr::from_ptr(nm_connection_get_connection_type(self.handle())) }
    }
}
