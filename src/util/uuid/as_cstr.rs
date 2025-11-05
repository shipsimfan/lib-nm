use crate::util::UUID;
use std::ffi::CStr;

impl UUID {
    /// Get the [`UUID`] as a [`CStr`]
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}
