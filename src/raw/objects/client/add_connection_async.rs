use crate::raw::{NMClient, NMConnection};
use glib_2::raw::{
    gio::{GAsyncReadyCallback, GCancellable},
    glib::{gboolean, gpointer},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMRemoteConnection;
#[allow(unused_imports)]
use glib_2::raw::glib::TRUE;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Requests that the remote settings service add the given settings to a new connection. If
    /// `save_to_disk` is [`TRUE`], the connection is immediately written to disk; otherwise it is
    /// initially only stored in memory, but may be saved later by calling the connection's
    /// [`nm_remote_connection_commit_changes`] method.
    ///
    /// `connection` is untouched by this function and only serves as a template of the settings to
    /// add. The [`NMRemoteConnection`] object that represents what NetworkManager actually added
    /// is returned to callback when the addition operation is complete.
    ///
    /// Note that the [`NMRemoteConnection`] returned in callback may not contain identical
    /// settings to connection as NetworkManager may perform automatic completion and/or
    /// normalization of connection properties.
    ///
    /// # Parameters
    ///  * `client` - the [`NMClient`]
    ///  * `connection` - the connection to add. Note that this object's settings will be added,
    ///                   not the object itself
    ///  * `save_to_disk` - whether to immediately save the connection to disk
    ///  * `cancellable` - a [`GCancellable`], or [`null_mut`]
    ///  * `callback` - callback to be called when the add operation completes.
    ///  * `user_data` - caller-specific data passed to `callback`
    pub fn nm_client_add_connection_async(
        client: *mut NMClient,
        connection: *mut NMConnection,
        save_to_disk: gboolean,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
}
