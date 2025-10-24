use crate::NMClient;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets NetworkManager version.
    ///
    /// # Parameters
    ///  * `client` - a [`NMClient`]
    ///
    /// # Returns
    ///  string with the version (or [`null_mut`] if NetworkManager is not running)
    pub fn nm_client_get_version(client: *mut NMClient) -> *const c_char;
}
