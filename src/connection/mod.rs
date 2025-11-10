use glib_2::gobject::GObject;

mod add_setting;
mod connection_type;
mod deref;
mod id;
mod new;
mod remove_setting;
mod setting_connection;
mod setting_ip4_config;
mod setting_wireless;
mod setting_wireless_security;
mod uuid;

/// Describes a connection to specific network or provider
#[derive(Clone)]
pub struct NMConnection {
    /// The handle to the underlying object
    object: GObject,
}
