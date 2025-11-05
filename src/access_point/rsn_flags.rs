use crate::{
    NMAccessPoint,
    raw::{NM80211ApSecurityFlags, nm_access_point_get_rsn_flags},
};

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Gets the RSN (Robust Secure Network, ie WPA version 2) flags of the access point.
    pub fn rsn_flags(&self) -> NM80211ApSecurityFlags {
        unsafe { nm_access_point_get_rsn_flags(self.handle) }
    }
}
