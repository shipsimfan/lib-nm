use std::ffi::CStr;

/// Indicates Ad-Hoc mode where no access point is expected to be present.
pub const NM_SETTING_WIRELESS_MODE_ADHOC: &CStr = c"adhoc";

/// Indicates AP/master mode where the wireless device is started as an access point/hotspot.
pub const NM_SETTING_WIRELESS_MODE_AP: &CStr = c"ap";

/// Indicates infrastructure mode where an access point is expected to be present for this
/// connection.
pub const NM_SETTING_WIRELESS_MODE_INFRA: &CStr = c"infrastructure";

/// Indicates that the connection should create a mesh point.
pub const NM_SETTING_WIRELESS_MODE_MESH: &CStr = c"mesh";
