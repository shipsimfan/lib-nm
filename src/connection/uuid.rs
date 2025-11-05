use crate::{NMConnection, raw::nm_connection_get_uuid};
use std::{borrow::Cow, ffi::CStr};

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSettingConnection;

impl NMConnection {
    /// A shortcut to return the UUID from the connection's [`NMSettingConnection`]
    pub fn uuid(&self) -> Cow<str> {
        self.uuid_raw().to_string_lossy()
    }

    /// A shortcut to return the UUID from the connection's [`NMSettingConnection`] as a raw
    /// [`CStr`]
    pub fn uuid_raw(&self) -> &CStr {
        unsafe { CStr::from_ptr(nm_connection_get_uuid(self.handle())) }
    }
}
