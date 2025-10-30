use crate::raw::NMAccessPoint;
use std::ffi::c_char;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the Basic Service Set ID (BSSID) of the Wi-Fi access point.
    ///
    /// # Parameters
    ///  * `ap` - a [`NMAccessPoint`]
    ///
    /// # Returns
    ///  the BSSID of the access point. This is an internal string and must not be modified or
    /// freed.
    pub fn nm_access_point_get_bssid(ap: *mut NMAccessPoint) -> *const c_char;
}
