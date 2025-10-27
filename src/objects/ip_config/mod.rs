use std::ffi::c_void;

mod get_addresses;
mod get_gateway;
mod get_nameservers;

pub use get_addresses::nm_ip_config_get_addresses;
pub use get_gateway::nm_ip_config_get_gateway;
pub use get_nameservers::nm_ip_config_get_nameservers;

/// [`NMIPConfig`]
pub type NMIPConfig = c_void;
