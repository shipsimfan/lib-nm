use crate::raw::NMRemoteConnection;
use glib_2::raw::{
    gio::{GAsyncReadyCallback, GCancellable},
    glib::gpointer,
};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Asynchronously deletes the connection.
    ///
    /// # Parameters
    ///  * `connection` - the [`NMRemoteConnection`]
    ///  * `cancellable` - a [`GCancellable`], or [`null_mut`]
    ///  * `callback` - callback to be called when the delete operation completes
    ///  * `user_data` - caller-specific data passed to `callback`
    pub fn nm_remote_connection_delete_async(
        connection: *mut NMRemoteConnection,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
}
