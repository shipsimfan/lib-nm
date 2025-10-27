use std::ffi::c_void;

mod get_address;
mod get_prefix;

pub use get_address::nm_ip_address_get_address;
pub use get_prefix::nm_ip_address_get_prefix;

#[allow(missing_docs)]
pub type NMIPAddress = c_void;
