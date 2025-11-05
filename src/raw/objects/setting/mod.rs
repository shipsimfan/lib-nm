use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMConnection;

/// Describes related configuration information
///
/// Each [`NMSetting`] contains properties that describe configuration that applies to a specific
/// network layer (like IPv4 or IPv6 configuration) or device type (like Ethernet, or Wi-Fi). A
/// collection of individual settings together make up an [`NMConnection`]. Each property is
/// strongly typed and usually has a number of allowed values. See each [`NMSetting`] subclass for
/// a description of properties and allowed values.
pub type NMSetting = c_void;
