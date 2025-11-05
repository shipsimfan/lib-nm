use crate::raw::{NMActiveConnection, NMClient};
use glib_2::raw::{
    gio::{GAsyncReadyCallback, GCancellable},
    glib::gpointer,
};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Asynchronously deactivates an active [`NMActiveConnection`].
    ///
    /// # Parameters
    ///  * `client` - a [`NMClient`]
    ///  * `active` - the [`NMActiveConnection`] to deactivate
    ///  * `cancellable` - a [`GCancellable`], or [`null_mut`]
    ///  * `callback` - callback to be called when the deactivation has completed
    ///  * `user_data` - caller-specific data passed to `callback`
    pub fn nm_client_deactivate_connection_async(
        client: *mut NMClient,
        active: *mut NMActiveConnection,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
}
