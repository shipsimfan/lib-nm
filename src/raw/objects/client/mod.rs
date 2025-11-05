use std::ffi::c_void;

mod activate_connection_async;
mod activate_connection_finish;
mod add_connection_async;
mod add_connection_finish;
mod deactivate_connection_async;
mod deactivate_connection_finish;
mod get_active_connections;
mod get_connection_by_id;
mod get_connections;
mod get_devices;
mod get_version;
mod new;

pub use activate_connection_async::nm_client_activate_connection_async;
pub use activate_connection_finish::nm_client_activate_connection_finish;
pub use add_connection_async::nm_client_add_connection_async;
pub use add_connection_finish::nm_client_add_connection_finish;
pub use deactivate_connection_async::nm_client_deactivate_connection_async;
pub use deactivate_connection_finish::nm_client_deactivate_connection_finish;
pub use get_active_connections::nm_client_get_active_connections;
pub use get_connection_by_id::nm_client_get_connection_by_id;
pub use get_connections::nm_client_get_connections;
pub use get_devices::nm_client_get_devices;
pub use get_version::nm_client_get_version;
pub use new::nm_client_new;

/// NMClient contains a cache of the objects of NetworkManager's D-Bus API. It uses `#GMainContext`
/// and `#GDBusConnection` for that and registers to D-Bus signals. That means, when iterating the
/// associated `#GMainContext`, D-Bus signals gets processed and the `#NMClient` instance updates
/// and emits `#GObject` signals.
pub type NMClient = c_void;
