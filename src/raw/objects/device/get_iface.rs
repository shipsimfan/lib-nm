use crate::raw::NMDevice;
use std::ffi::c_char;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the interface name of the [`NMDevice`].
    ///
    /// # Parameters
    ///  * `device` - a [`NMDevice`]
    ///
    /// # Returns
    /// the interface of the device. This is the internal string used by the device, and must not
    /// be modified.
    pub fn nm_device_get_iface(device: *mut NMDevice) -> *const c_char;
}
