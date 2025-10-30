use crate::raw::NMRemoteConnection;
use glib_2::{
    gio::GAsyncResult,
    glib::{GError, gboolean},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::nm_remote_connection_commit_changes_async;
#[allow(unused_imports)]
use glib_2::{
    gio::GAsyncReadyCallback,
    glib::{FALSE, TRUE},
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the result of a call to [`nm_remote_connection_commit_changes_async`].
    ///
    /// # Parameters
    ///  * `connection` - the [`NMRemoteConnection`]
    ///  * `result` - the result passed to the [`GAsyncReadyCallback`]
    ///  * `error` - location for a [`GError`], or [`null_mut`]
    ///
    /// # Returns
    /// [`TRUE`] on success, [`FALSE`] on error, in which case error will be set.
    pub fn nm_remote_connection_commit_changes_finish(
        connection: *mut NMRemoteConnection,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
}
