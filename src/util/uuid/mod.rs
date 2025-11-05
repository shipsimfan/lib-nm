use std::ffi::c_char;

mod as_cstr;
mod as_ptr;
mod drop;
mod generate;

/// An owned universally unique identifier returned from libnm
pub struct UUID {
    /// The underlying pointer to the UUID string
    ptr: *mut c_char,
}
