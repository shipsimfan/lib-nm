use crate::raw::NMRemoteConnection;
use glib_2::raw::{
    gio::GAsyncResult,
    glib::{GError, gboolean},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::nm_remote_connection_delete_async;
#[allow(unused_imports)]
use glib_2::raw::{
    gio::GAsyncReadyCallback,
    glib::{FALSE, TRUE},
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the result of a call to [`nm_remote_connection_delete_async`].
    ///
    /// # Parameters
    ///  * `connection` - the [`NMRemoteConnection`]
    ///  * `result` - the result passed to the [`GAsyncReadyCallback`]
    ///  * `error` - location for a [`GError`], or [`null_mut`]
    ///
    /// # Returns
    /// [`TRUE`] on success, [`FALSE`] on error, in which case error will be set.
    pub fn nm_remote_connection_delete_finish(
        connection: *mut NMRemoteConnection,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
}
