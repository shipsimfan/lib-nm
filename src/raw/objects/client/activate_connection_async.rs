use crate::raw::{NMClient, NMConnection, NMDevice};
use glib_2::raw::{
    gio::{GAsyncReadyCallback, GCancellable},
    glib::gpointer,
};
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMAccessPoint, NMActiveConnection};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Asynchronously starts a connection to a particular network using the configuration settings
    /// from `connection` and the network device `device`. Certain connection types also take a
    /// "specific object" which is the object path of a connection- specific object, like an
    /// [`NMAccessPoint`] for Wi-Fi connections, or an [`NMWimaxNsp`] for WiMAX connections, to
    /// which you wish to connect. If the specific object is not given, NetworkManager can, in some
    /// cases, automatically determine which network to connect to given the settings in
    /// `connection`.
    ///
    /// If `connection` is not given for a device-based activation, NetworkManager picks the best
    /// available connection for the device and activates it.
    ///
    /// Note that the callback is invoked when NetworkManager has started activating the new
    /// connection, not when it finishes. You can use the returned [`NMActiveConnection`] object
    /// (in particular, “state”) to track the activation to its completion.
    ///
    /// # Parameters
    ///  * `client` - a [`NMClient`]
    ///  * `connection` - an [`NMConnection`]. This parameter can be [`null_mut`].
    ///  * `device` - the [`NMDevice`]. This parameter can be [`null_mut`].
    ///  * `specific_object` - the object path of a connection-type-specific object this activation
    ///                        should use. This parameter is currently ignored for wired and mobile
    ///                        broadband connections, and the value of [`null_mut`] should be used
    ///                        (ie, no specific object). For Wi-Fi or WiMAX connections, pass the
    ///                        object path of a [`NMAccessPoint`] or [`NMWimaxNsp`] owned by
    ///                        `device`, which you can get using [`nm_object_get_path`], and which
    ///                        will be used to complete the details of the newly added connection.
    ///                        This parameter can be [`null_mut`].
    ///  * `cancellable` - a [`GCancellable`], or [`null_mut`]
    ///  * `callback` - callback to be called when the activation has started
    ///  * `user_data` - caller-specific data passed to `callback`
    pub fn nm_client_activate_connection_async(
        client: *mut NMClient,
        connection: *mut NMConnection,
        device: *mut NMDevice,
        specific_object: *const c_char,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
}
