use crate::raw::{NM80211ApSecurityFlags, NMAccessPoint};

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the WPA (version 1) flags of the access point.
    ///
    /// # Parameters
    ///  * `ap` - a [`NMAccessPoint`]
    ///
    /// # Returns
    /// the WPA flags
    pub fn nm_access_point_get_wpa_flags(ap: *mut NMAccessPoint) -> NM80211ApSecurityFlags;
}
