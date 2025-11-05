use crate::raw::NMDevice;
use glib_2::raw::glib::GPtrArray;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMConnection, nm_client_get_connections};
#[allow(unused_imports)]
use glib_2::raw::glib::g_ptr_array_unref;

#[link(name = "nm")]
unsafe extern "C" {
    /// Filters a given array of connections for a given [`NMDevice`] object and returns
    /// connections which may be activated with the device. For example if device is a Wi-Fi device
    /// that supports only WEP encryption, the returned array will contain any Wi-Fi connections in
    /// connections that allow connection to unencrypted or WEP-enabled SSIDs. The returned array
    /// will not contain Ethernet, Bluetooth, Wi-Fi WPA connections, or any other connection that
    /// is incompatible with the device. To get the full list of connections see
    /// [`nm_client_get_connections`].
    ///
    /// # Parameters
    ///  * `device` - an [`NMDevice`] to filter connections for
    ///  * `connections` - an array of [`NMConnection`]s to filter.
    ///
    /// # Returns
    /// an array of [`NMConnection`]s that could be activated with the given device. The array
    /// should be freed with [`g_ptr_array_unref`] when it is no longer required.
    pub fn nm_device_filter_connections(
        device: *mut NMDevice,
        connections: *const GPtrArray,
    ) -> *mut GPtrArray;
}
