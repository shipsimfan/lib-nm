use std::ffi::c_void;

mod connection_state_reasons;
mod connection_states;
mod get_connection;
mod get_connection_type;
mod get_id;
mod get_ip4_config;

pub use connection_state_reasons::*;
pub use connection_states::*;
pub use get_connection::nm_active_connection_get_connection;
pub use get_connection_type::nm_active_connection_get_connection_type;
pub use get_id::nm_active_connection_get_id;
pub use get_ip4_config::nm_active_connection_get_ip4_config;

#[allow(missing_docs)]
pub type NMActiveConnection = c_void;
