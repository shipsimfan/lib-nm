use crate::NMSetting;

mod deref;
mod get_gtype;
mod mode;
mod new;
mod ssid;

/// Describes connection properties for 802.11 Wi-Fi networks
pub struct NMSettingWireless<'connection> {
    /// The underlying setting
    settings: NMSetting<'connection>,
}
