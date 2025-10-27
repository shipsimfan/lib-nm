use std::ffi::c_void;

mod get_iface;

pub use get_iface::nm_device_get_iface;

/// [`NMDevice`]
pub type NMDevice = c_void;
