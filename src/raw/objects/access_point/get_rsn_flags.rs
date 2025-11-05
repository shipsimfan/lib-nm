use crate::raw::{NM80211ApSecurityFlags, NMAccessPoint};

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the RSN (Robust Secure Network, ie WPA version 2) flags of the access point.
    ///
    /// # Parameters
    ///  * `ap` - a [`NMAccessPoint`]
    ///
    /// # Returns
    /// the RSN flags
    pub fn nm_access_point_get_rsn_flags(ap: *mut NMAccessPoint) -> NM80211ApSecurityFlags;
}
