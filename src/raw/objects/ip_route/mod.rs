use std::ffi::c_void;

mod new_binary;
mod unref;

pub use new_binary::nm_ip_route_new_binary;
pub use unref::nm_ip_route_unref;

#[allow(missing_docs)]
pub type NMIPRoute = c_void;
