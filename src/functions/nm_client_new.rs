use crate::NMClient;
use glib_2::{gio::GCancellable, glib::GError};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMClient`] synchronously.
    ///
    /// Note that this will block until a [`NMClient`] instance is fully initialized. This does
    /// nothing beside calling [`g_initable_new`]. You are free to call [`g_initable_new`] or
    /// [`g_object_new`]/[`g_initable_init`] directly for more control, to set [`GObject`]
    /// properties or get access to the [`NMClient`] instance while it is still initializing.
    ///
    /// Using the synchronous initialization creates an [`NMClient`] instance that uses an internal
    /// [`GMainContext`]. This context is invisible to the user. This introduces an additional
    /// overhead that is payed not only during object initialization, but for the entire lifetime
    /// of this object. Also, due to this internal [`GMainContext`], the events are no longer in
    /// sync with other messages from [`GDBusConnection`] (but all events of the [`NMClient`] will
    /// themselves still be ordered). For a serious program, you should therefore avoid these
    /// problems by using [`g_async_initable_init_async`] or [`nm_client_new_async`] instead. The
    /// sync initialization is still useful for simple scripts or interactive testing for example
    /// via `pygobject`.
    ///
    /// Creating an [`NMClient`] instance can only fail for two reasons. First, if you didn't
    /// provide a [`NM_CLIENT_DBUS_CONNECTION`] and the call to [`g_bus_get`] fails. You can avoid
    /// that by using [`g_initable_new`] directly and set a D-Bus connection. Second, if you
    /// cancelled the creation. If you do that, then note that after the failure there might still
    /// be idle actions pending which keep [`nm_client_get_main_context`] alive. That means, in
    /// that case you must continue iterating the context to avoid leaks. See
    /// [`nm_client_get_context_busy_watcher`].
    ///
    /// Creating an [`NMClient`] instance when NetworkManager is not running does not cause a
    /// failure.
    ///
    /// # Parameters
    ///  * `cancellable` - a [`GCancellable`], or [`null_mut`]
    ///  * `error` - location for a [`GError`], or [`null_mut`]
    ///
    /// # Returns
    ///  a new [`NMClient`] or [`null_mut`] on an error
    pub fn nm_client_new(cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut NMClient;
}
