use std::ffi::c_void;

mod get_device_type;
mod get_iface;

pub use get_device_type::nm_device_get_device_type;
pub use get_iface::nm_device_get_iface;

/// [`NMDevice`]
pub type NMDevice = c_void;
