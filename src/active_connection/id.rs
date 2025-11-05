use crate::{NMActiveConnection, raw::nm_active_connection_get_id};
use std::{borrow::Cow, ffi::CStr};

impl<'owner, Owner> NMActiveConnection<'owner, Owner> {
    /// Gets the [`NMActiveConnection`]'s ID
    pub fn id(&self) -> Cow<str> {
        self.id_raw().to_string_lossy()
    }

    /// Gets the [`NMActiveConnection`]'s ID as a raw [`CStr`]
    pub fn id_raw(&self) -> &CStr {
        unsafe { CStr::from_ptr(nm_active_connection_get_id(self.handle())) }
    }
}
