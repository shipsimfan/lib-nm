use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMConnection, NMSetting};

mod get_type;
mod new;
mod properties;

pub use get_type::nm_setting_connection_get_type;
pub use new::nm_setting_connection_new;
pub use properties::*;

/// Describes general connection properties
///
/// The [`NMSettingConnection`] object is a [`NMSetting`] subclass that describes properties that
/// apply to all [`NMConnection`] objects, regardless of what type of network connection they
/// describe. Each [`NMConnection`] object must contain a [`NMSettingConnection`] setting.
pub type NMSettingConnection = c_void;
