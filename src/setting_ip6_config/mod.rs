use crate::NMSettingIPConfig;

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSetting;

mod deref;
mod get_gtype;
mod new;

/// Describes IPv6 addressing, routing, and name service properties
pub struct NMSettingIP6Config<'connection> {
    /// The underlying IP config setting
    ip_config_settings: NMSettingIPConfig<'connection>,
}
