use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSetting;

mod get_ssid;
mod get_type;
mod modes;
mod new;
mod properties;

pub use get_ssid::nm_setting_wireless_get_ssid;
pub use get_type::nm_setting_wireless_get_type;
pub use modes::*;
pub use new::nm_setting_wireless_new;
pub use properties::*;

/// Describes connection properties for 802.11 Wi-Fi networks
///
/// The [`NMSettingWireless`] object is a [`NMSetting`] subclass that describes properties
/// necessary for connection to 802.11 Wi-Fi networks.
pub type NMSettingWireless = c_void;
