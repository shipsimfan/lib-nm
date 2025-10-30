use crate::raw::NMIPAddress;
use glib_2::raw::glib::{GError, gconstpointer, guint};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMIPAddress`] object. `addr` must point to a buffer of the correct size for
    /// `family`.
    ///
    /// # Parameters
    ///  * `family` - the IP address family (`AF_INET` or `AF_INET6`)
    ///  * `addr` - the IP address
    ///  * `prefix` - the address prefix length
    ///  * `error` - location to store error, or [`null_mut`]
    ///
    /// # Returns
    /// the new [`NMIPAddress`] object, or [`null_mut`] on error.
    pub fn nm_ip_address_new_binary(
        family: c_int,
        addr: gconstpointer,
        prefix: guint,
        error: *mut *mut GError,
    ) -> *mut NMIPAddress;
}
