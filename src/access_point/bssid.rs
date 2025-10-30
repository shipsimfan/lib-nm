use crate::{NMAccessPoint, raw::nm_access_point_get_bssid};
use std::{borrow::Cow, ffi::CStr};

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Gets the Basic Service Set ID (BSSID) of the Wi-Fi access point
    pub fn bssid(&self) -> Cow<str> {
        self.bssid_raw().to_string_lossy()
    }

    /// Gets the Basic Service Set ID (BSSID) of the Wi-Fi access point as a raw [`CStr`]
    pub fn bssid_raw(&self) -> &CStr {
        unsafe { CStr::from_ptr(nm_access_point_get_bssid(self.handle)) }
    }
}
