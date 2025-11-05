use glib_2::gobject::GObject;

mod activate_connection;
mod add_connection;
mod connections;
mod deactivate_connection;
mod deref;
mod devices;
mod new;
mod version;

/// [`NMClient`] contains a cache of the objects of NetworkManager's D-Bus API.
pub struct NMClient {
    /// The handle to underlying object
    object: GObject,
}
