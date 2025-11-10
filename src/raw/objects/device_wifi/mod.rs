use std::ffi::c_void;

mod get_access_points;
mod get_active_access_point;

pub use get_access_points::nm_device_wifi_get_access_points;
pub use get_active_access_point::nm_device_wifi_get_active_access_point;

#[allow(missing_docs)]
pub type NMDeviceWifi = c_void;
