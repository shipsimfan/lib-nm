use std::ffi::c_void;

mod filter_connections;
mod get_active_connection;
mod get_device_type;
mod get_iface;

pub use filter_connections::nm_device_filter_connections;
pub use get_active_connection::nm_device_get_active_connection;
pub use get_device_type::nm_device_get_device_type;
pub use get_iface::nm_device_get_iface;

#[allow(missing_docs)]
pub type NMDevice = c_void;
