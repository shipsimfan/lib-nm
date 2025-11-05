use std::ffi::c_void;

mod filter_connections;
mod get_bssid;
mod get_max_bitrate;
mod get_rsn_flags;
mod get_ssid;
mod get_strength;
mod get_wpa_flags;

pub use filter_connections::nm_access_point_filter_connections;
pub use get_bssid::nm_access_point_get_bssid;
pub use get_max_bitrate::nm_access_point_get_max_bitrate;
pub use get_rsn_flags::nm_access_point_get_rsn_flags;
pub use get_ssid::nm_access_point_get_ssid;
pub use get_strength::nm_access_point_get_strength;
pub use get_wpa_flags::nm_access_point_get_wpa_flags;

#[allow(missing_docs)]
pub type NMAccessPoint = c_void;
