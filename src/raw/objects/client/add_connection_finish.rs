use crate::raw::{NMClient, NMRemoteConnection};
use glib_2::raw::{gio::GAsyncResult, glib::GError};

// rustdoc imports
#[allow(unused_imports)]
use glib_2::raw::gio::GAsyncReadyCallback;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the result of a call to [`nm_client_add_connection_async`].
    ///
    /// # Parameters
    ///  * `client` - an [`NMClient`]
    ///  * `result` - the result passed to the [`GAsyncReadyCallback`]
    ///  * `error` - location for a [`GError`], or [`null_mut`]
    ///
    /// # Returns
    /// the new [`NMRemoteConnection`] on success, [`null_mut`] on failure, in which case error
    /// will be set.
    pub fn nm_client_add_connection_finish(
        client: *mut NMClient,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut NMRemoteConnection;
}
