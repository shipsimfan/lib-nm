use crate::{
    NMAccessPoint,
    raw::{NM80211ApSecurityFlags, nm_access_point_get_wpa_flags},
};

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Gets the WPA (version 1) flags of the access point.
    pub fn wpa_flags(&self) -> NM80211ApSecurityFlags {
        unsafe { nm_access_point_get_wpa_flags(self.handle) }
    }
}
