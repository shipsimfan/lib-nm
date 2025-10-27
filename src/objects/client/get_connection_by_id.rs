use crate::{NMClient, NMRemoteConnection};
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::nm_connection_verify;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Returns the first matching [`NMRemoteConnection`] matching a given `id`.
    ///
    /// # Parameters
    ///  * `client` - the [`NMClient`]
    ///  * `id` - the id of the remote connection
    ///
    /// # Returns
    /// the remote connection object on success, or [`null_mut`] if no matching object was found.
    ///
    /// The connection is as received from D-Bus and might not validate according to
    /// [`nm_connection_verify`].
    pub fn nm_client_get_connection_by_id(
        client: *mut NMClient,
        id: *const c_char,
    ) -> *mut NMRemoteConnection;
}
