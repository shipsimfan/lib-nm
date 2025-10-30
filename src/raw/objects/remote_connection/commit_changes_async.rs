use crate::raw::NMRemoteConnection;
use glib_2::raw::{
    gio::{GAsyncReadyCallback, GCancellable},
    glib::{gboolean, gpointer},
};

// rustdoc imports
#[allow(unused_imports)]
use glib_2::raw::glib::{FALSE, TRUE};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Asynchronously sends any local changes to the settings and properties of connection to
    /// NetworkManager. If save is [`TRUE`], the updated connection will be saved to disk; if
    /// [`FALSE`], then only the in-memory representation will be changed.
    ///
    /// # Parameters
    ///  * `connection` - the [`NMRemoteConnection`]
    ///  * `save_to_disk` - whether to save the changes to persistent storage
    ///  * `cancellable` - a [`GCancellable`], or [`null_mut`]
    ///  * `callback` - callback to be called when the commit operation completes
    ///  * `user_data` - caller-specific data passed to `callback`
    pub fn nm_remote_connection_commit_changes_async(
        connection: *mut NMRemoteConnection,
        save_to_disk: gboolean,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
}
