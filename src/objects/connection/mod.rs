use std::ffi::c_void;

mod get_connection_type;
mod get_id;
mod get_setting_ip4_config;

pub use get_connection_type::nm_connection_get_connection_type;
pub use get_id::nm_connection_get_id;
pub use get_setting_ip4_config::nm_connection_get_setting_ip4_config;

/// Describes a connection to specific network or provider
pub type NMConnection = c_void;
