use crate::raw::NMAccessPoint;
use glib_2::raw::glib::GPtrArray;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMConnection, NMDevice, nm_client_get_connections, nm_device_filter_connections};
#[allow(unused_imports)]
use glib_2::raw::glib::g_ptr_array_unref;

#[link(name = "nm")]
unsafe extern "C" {
    /// Filters a given array of connections for a given [`NMAccessPoint`] object and returns
    /// connections which may be activated with the access point. Any returned connections will
    /// match the `ap`'s SSID and (if given) BSSID and other attributes like security settings,
    /// channel, etc.
    ///
    /// To obtain the list of connections that are compatible with this access point, use
    /// [`nm_client_get_connections`] and then filter the returned list for a given [`NMDevice`]
    /// using [`nm_device_filter_connections`] and finally filter that list with this function.
    ///
    /// # Parameters
    ///  * `ap` - an [`NMAccessPoint`] to filter connections for
    ///  * `connections` - an array of [`NMConnection`]s to filter.
    ///
    /// # Returns
    /// an array of [`NMConnection`]s that could be activated with the given `ap`. The array should
    /// be freed with [`g_ptr_array_unref`] when it is no longer required.
    pub fn nm_access_point_filter_connections(
        ap: *mut NMAccessPoint,
        connections: *const GPtrArray,
    ) -> *mut GPtrArray;
}
