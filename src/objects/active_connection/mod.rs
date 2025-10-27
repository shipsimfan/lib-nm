use std::ffi::c_void;

mod get_connection_type;
mod get_id;
mod get_ip4_config;

pub use get_connection_type::nm_active_connection_get_connection_type;
pub use get_id::nm_active_connection_get_id;
pub use get_ip4_config::nm_active_connection_get_ip4_config;

#[allow(missing_docs)]
pub type NMActiveConnection = c_void;
