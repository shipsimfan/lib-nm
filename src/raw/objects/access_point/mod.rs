use std::ffi::c_void;

mod get_max_bitrate;
mod get_ssid;
mod get_strength;

pub use get_max_bitrate::nm_access_point_get_max_bitrate;
pub use get_ssid::nm_access_point_get_ssid;
pub use get_strength::nm_access_point_get_strength;

/// [`NMAccessPoint`]
pub type NMAccessPoint = c_void;
