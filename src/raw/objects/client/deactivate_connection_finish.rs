use crate::raw::NMClient;
use glib_2::raw::{
    gio::GAsyncResult,
    glib::{GError, gboolean},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::nm_client_deactivate_connection_async;
#[allow(unused_imports)]
use glib_2::raw::gio::GAsyncReadyCallback;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the result of a call to [`nm_client_deactivate_connection_async`].
    ///
    /// # Parameters
    ///  * `client` - a [`NMClient`]
    ///  * `result` - the result passed to the [`GAsyncReadyCallback`]
    ///  * `error` - location for a [`GError`], or [`null_mut`]
    ///
    /// # Returns
    /// success or failure
    pub fn nm_client_deactivate_connection_finish(
        client: *mut NMClient,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
}
