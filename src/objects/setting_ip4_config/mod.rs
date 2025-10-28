use std::ffi::c_void;

mod new;

pub use new::nm_setting_ip4_config_new;

/// Describes IPv4 addressing, routing, and name service properties
pub type NMSettingIP4Config = c_void;
