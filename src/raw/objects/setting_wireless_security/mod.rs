use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSetting;

mod get_type;
mod new;
mod properties;

pub use get_type::nm_setting_wireless_security_get_type;
pub use new::nm_setting_wireless_security_new;
pub use properties::*;

/// Describes connection properties for Wi-Fi networks that use WEP, LEAP, WPA or WPA2/RSN security
///
/// The [`NMSettingWirelessSecurity`] object is a [`NMSetting`] subclass that describes properties
/// necessary for connection to encrypted Wi-Fi networks.
///
/// It's a good idea to read up on `wpa_supplicant` configuration before using this setting
/// extensively, since most of the options here correspond closely with the relevant
/// `wpa_supplicant` configuration options. To get a better overview of how Wi-Fi security works,
/// you may want to get copies of the following books:
///  - "802.11 Wireless Networks: The Definitive Guide, Second Edition" Author: Matthew Gast ISBN:
///    978-0596100520
///  - "Cisco Wireless LAN Security" Authors: Krishna Sankar, Sri Sundaralingam, Darrin Miller, and
///    Andrew Balinsky ISBN: 978-1587051548
pub type NMSettingWirelessSecurity = c_void;
