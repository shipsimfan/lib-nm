use std::ffi::c_void;

mod new_binary;

pub use new_binary::nm_ip_route_new_binary;

#[allow(missing_docs)]
pub type NMIPRoute = c_void;
