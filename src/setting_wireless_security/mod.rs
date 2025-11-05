use crate::NMSetting;

mod deref;
mod get_gtype;
mod key_mgmt;
mod new;
mod psk;

/// Describes connection properties for Wi-Fi networks that use WEP, LEAP, WPA or WPA2/RSN security
pub struct NMSettingWirelessSecurity<'connection> {
    /// The underlying setting
    settings: NMSetting<'connection>,
}
