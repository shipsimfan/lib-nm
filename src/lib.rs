//! libnm (Network Manager) bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod raw;
pub mod util;

pub use raw::NMDeviceType;

mod access_point;
mod active_connection;
mod client;
mod connection;
mod device;
mod device_wifi;
mod ip_address;
mod remote_connection;
mod setting;
mod setting_connection;
mod setting_ip4_config;
mod setting_ip6_config;
mod setting_ip_config;
mod setting_wireless;
mod setting_wireless_security;
mod simple_connection;

pub use access_point::{NMAccessPoint, NMAccessPointIter};
pub use active_connection::{NMActiveConnection, NMActiveConnectionStateChangedCallback};
pub use client::NMClient;
pub use connection::NMConnection;
pub use device::{NMDevice, NMDeviceIter};
pub use device_wifi::NMDeviceWifi;
pub use ip_address::NMIPAddress;
pub use remote_connection::{NMRemoteConnection, NMRemoteConnectionIter};
pub use setting::NMSetting;
pub use setting_connection::NMSettingConnection;
pub use setting_ip_config::NMSettingIPConfig;
pub use setting_ip4_config::NMSettingIP4Config;
pub use setting_ip6_config::NMSettingIP6Config;
pub use setting_wireless::NMSettingWireless;
pub use setting_wireless_security::NMSettingWirelessSecurity;
pub use simple_connection::NMSimpleConnection;
