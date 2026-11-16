use crate::raw::NMDevice;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the current a hardware address (MAC) for the `device`.
    ///
    /// # Parameters
    ///  * `device` - a [`NMDevice`]
    ///
    /// # Returns
    /// the current MAC of the device, or [`null`]. This is the internal string used by the device,
    /// and must not be modified.
    pub fn nm_device_get_hw_address(device: *mut NMDevice) -> *const c_char;
}
