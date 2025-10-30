use crate::raw::NMIPRoute;
use glib_2::raw::glib::{GError, gconstpointer, gint64, guint};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMIPRoute`] object. `dest` and `next_hop` (if not [`null`]) must point to
    /// buffers of the correct size for `family`.
    ///
    /// # Parameters
    ///  * `family` - the IP address family (`AF_INET` or `AF_INET6`)
    ///  * `dest` - the IP address of the route's destination
    ///  * `prefix` - the address prefix length
    ///  * `next_hop` - the IP address of the next hop (or [`null`]).
    ///  * `metric` - the route metric (or -1 for "default")
    ///  * `error` - location to store error, or [`null_mut`]
    ///
    /// # Returns
    /// the new [`NMIPRoute`] object, or [`null_mut`] on error.
    pub fn nm_ip_route_new_binary(
        family: c_int,
        dest: gconstpointer,
        prefix: guint,
        next_hop: gconstpointer,
        metric: gint64,
        error: *mut *mut GError,
    ) -> *mut NMIPRoute;
}
