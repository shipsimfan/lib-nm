use std::ffi::c_void;

mod new;

pub use new::nm_simple_connection_new;

/// An unmanaged connection
///
/// An [`NMSimpleConnection`] does not directly represent a D-Bus-exported connection, but might be
/// used in the process of creating a new one.
pub type NMSimpleConnection = c_void;
