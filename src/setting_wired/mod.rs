use crate::NMSetting;

mod deref;
mod get_gtype;
mod mac_address;
mod new;

/// Describes connection properties for wired networks
pub struct NMSettingWired<'connection> {
    /// The underlying setting
    settings: NMSetting<'connection>,
}
