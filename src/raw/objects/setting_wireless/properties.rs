use std::ffi::CStr;

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_SSID: &CStr = c"ssid";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_MODE: &CStr = c"mode";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_BAND: &CStr = c"band";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_CHANNEL: &CStr = c"channel";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_BSSID: &CStr = c"bssid";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_RATE: &CStr = c"rate";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_TX_POWER: &CStr = c"tx-power";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_MAC_ADDRESS: &CStr = c"mac-address";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_CLONED_MAC_ADDRESS: &CStr = c"cloned-mac-address";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_GENERATE_MAC_ADDRESS_MASK: &CStr = c"generate-mac-address-mask";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_MAC_ADDRESS_BLACKLIST: &CStr = c"mac-address-blacklist";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_MTU: &CStr = c"mtu";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_SEEN_BSSIDS: &CStr = c"seen-bssids";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_HIDDEN: &CStr = c"hidden";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_POWERSAVE: &CStr = c"powersave";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_MAC_ADDRESS_RANDOMIZATION: &CStr = c"mac-address-randomization";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_WAKE_ON_WLAN: &CStr = c"wake-on-wlan";

#[allow(missing_docs)]
pub const NM_SETTING_WIRELESS_AP_ISOLATION: &CStr = c"ap-isolation";
