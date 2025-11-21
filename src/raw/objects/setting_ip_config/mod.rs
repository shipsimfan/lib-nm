use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSettingIP4Config;

mod add_address;
mod add_dns;
mod add_route;
mod get_address;
mod get_dns;
mod get_gateway;
mod get_method;
mod get_num_addresses;
mod get_num_dns;
mod get_type;
mod properties;

pub use add_address::nm_setting_ip_config_add_address;
pub use add_dns::nm_setting_ip_config_add_dns;
pub use add_route::nm_setting_ip_config_add_route;
pub use get_address::nm_setting_ip_config_get_address;
pub use get_dns::nm_setting_ip_config_get_dns;
pub use get_gateway::nm_setting_ip_config_get_gateway;
pub use get_method::nm_setting_ip_config_get_method;
pub use get_num_addresses::nm_setting_ip_config_get_num_addresses;
pub use get_num_dns::nm_setting_ip_config_get_num_dns;
pub use get_type::nm_setting_ip_config_get_type;
pub use properties::*;

/// Abstract base class for IPv4 and IPv6 addressing, routing, and name service properties
///
/// [`NMSettingIPConfig`] is the abstract base class of [`NMSettingIP4Config`] and
/// [`NMSettingIP6Config`], providing properties related to IP addressing, routing, and Domain Name
/// Service.
pub type NMSettingIPConfig = c_void;
