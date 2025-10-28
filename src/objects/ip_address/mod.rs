use std::ffi::c_void;

mod get_address;
mod get_prefix;
mod new_binary;

pub use get_address::nm_ip_address_get_address;
pub use get_prefix::nm_ip_address_get_prefix;
pub use new_binary::nm_ip_address_new_binary;

#[allow(missing_docs)]
pub type NMIPAddress = c_void;
