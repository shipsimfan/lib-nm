use crate::NMSetting;

mod deref;
mod get_gtype;
mod method;
mod new;

/// Abstract base class for IPv4 and IPv6 addressing, routing, and name service properties
pub struct NMSettingIPConfig<'connection> {
    /// The underlying setting
    settings: NMSetting<'connection>,
}
