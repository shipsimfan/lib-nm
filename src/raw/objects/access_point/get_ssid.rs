use crate::raw::NMAccessPoint;
use glib_2::glib::GBytes;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the SSID of the access point.
    ///
    /// # Parameters
    ///  * `ap` - a [`NMAccessPoint`]
    ///
    /// # Returns
    /// the [`GBytes`] containing the SSID, or [`null_mut`] if the SSID is unknown.
    pub fn nm_access_point_get_ssid(ap: *mut NMAccessPoint) -> *mut GBytes;
}
