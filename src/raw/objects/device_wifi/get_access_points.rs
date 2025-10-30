use crate::raw::NMDeviceWifi;
use glib_2::raw::glib::GPtrArray;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMAccessPoint;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets all the scanned access points of the [`NMDeviceWifi`].
    ///
    /// # Parameters
    ///  * `device` - a [`NMDeviceWifi`]
    ///
    /// # Returns
    /// a [`GPtrArray`] containing all the scanned [`NMAccessPoint`]s. The returned array is owned
    /// by the client and should not be modified.
    pub fn nm_device_wifi_get_access_points(device: *mut NMDeviceWifi) -> *const GPtrArray;
}
