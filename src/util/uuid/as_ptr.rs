use crate::util::UUID;
use std::ffi::c_char;

impl UUID {
    /// Get the underlying pointer of the UUID
    pub const fn as_ptr(&self) -> *mut c_char {
        self.ptr
    }
}
