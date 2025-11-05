use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSetting;

mod add_setting;
mod clear_settings;
mod get_connection_type;
mod get_id;
mod get_setting_connection;
mod get_setting_ip4_config;
mod get_setting_wireless;
mod get_setting_wireless_security;
mod get_uuid;
mod remove_setting;
mod verify;

pub use add_setting::nm_connection_add_setting;
pub use clear_settings::nm_connection_clear_settings;
pub use get_connection_type::nm_connection_get_connection_type;
pub use get_id::nm_connection_get_id;
pub use get_setting_connection::nm_connection_get_setting_connection;
pub use get_setting_ip4_config::nm_connection_get_setting_ip4_config;
pub use get_setting_wireless::nm_connection_get_setting_wireless;
pub use get_setting_wireless_security::nm_connection_get_setting_wireless_security;
pub use get_uuid::nm_connection_get_uuid;
pub use remove_setting::nm_connection_remove_setting;
pub use verify::nm_connection_verify;

/// Describes a connection to specific network or provider
///
/// An [`NMConnection`] describes all the settings and configuration values that are necessary to
/// configure network devices for operation on a specific network. Connections are the fundamental
/// operating object for NetworkManager; no device is connected without a [`NMConnection`], or
/// disconnected without having been connected with a [`NMConnection`].
///
/// Each [`NMConnection`] contains a list of [`NMSetting`] objects usually referenced by name
/// (using [`nm_connection_get_setting_by_name`]) or by type (with [`nm_connection_get_setting`]).
/// The settings describe the actual parameters with which the network devices are configured,
/// including device-specific parameters (MTU, SSID, APN, channel, rate, etc) and IP-level
/// parameters (addresses, routes, addressing methods, etc).
pub type NMConnection = c_void;
