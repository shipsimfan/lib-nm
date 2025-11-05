use crate::{NMClient, raw::nm_client_get_version};
use std::{borrow::Cow, ffi::CStr, ptr::null};

impl NMClient {
    /// Get the version of NetworkManager this client is connected to, if it is
    pub fn version(&self) -> Option<Cow<str>> {
        self.version_raw().map(CStr::to_string_lossy)
    }

    /// Get the raw [`CStr`] of the version of NetworkManager connected to
    pub fn version_raw(&self) -> Option<&CStr> {
        let ptr = unsafe { nm_client_get_version(self.handle()) };

        if ptr == null() {
            return None;
        }

        Some(unsafe { CStr::from_ptr(ptr) })
    }
}
