use crate::raw::NMClient;
use glib_2::raw::glib::GPtrArray;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMDevice, nm_device_get_device_type};

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets all the known network devices. Use [`nm_device_get_device_type`] or the
    /// `NM_IS_DEVICE_XXXX` functions to determine what kind of device member of the returned array
    /// is, and then you may use device-specific methods such as
    /// [`nm_device_ethernet_get_hw_address`].
    ///
    /// # Parameters
    ///  * `client` - a [`NMClient`]
    ///
    /// # Returns
    /// a [`GPtrArray`] containing all the [`NMDevice`]s. The returned array is owned by the
    /// [`NMClient`] object and should not be modified.
    pub fn nm_client_get_devices(client: *mut NMClient) -> *const GPtrArray;
}
