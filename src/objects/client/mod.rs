use std::ffi::c_void;

mod get_active_connections;
mod get_connection_by_id;
mod get_connections;
mod get_version;

pub use get_active_connections::nm_client_get_active_connections;
pub use get_connection_by_id::nm_client_get_connection_by_id;
pub use get_connections::nm_client_get_connections;
pub use get_version::nm_client_get_version;

/// NMClient contains a cache of the objects of NetworkManager's D-Bus API. It uses `#GMainContext`
/// and `#GDBusConnection` for that and registers to D-Bus signals. That means, when iterating the
/// associated `#GMainContext`, D-Bus signals gets processed and the `#NMClient` instance updates
/// and emits `#GObject` signals.
pub type NMClient = c_void;
