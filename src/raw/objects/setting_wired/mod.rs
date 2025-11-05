use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSetting;

mod get_type;

pub use get_type::nm_setting_wired_get_type;

/// Describes connection properties for Ethernet-based networks
///
/// The [`NMSettingWired`] object is a [`NMSetting`] subclass that describes properties necessary
/// for connection to Ethernet networks.
pub type NMSettingWired = c_void;
