use crate::raw::{NMDevice, NMDeviceType};

#[link(name = "nm")]
unsafe extern "C" {
    /// Returns the numeric type of the [`NMDevice`], ie Ethernet, Wi-Fi, etc.
    ///
    /// # Parameters
    ///  * `device` - a [`NMDevice`]
    ///
    /// # Returns
    ///  the device type
    pub fn nm_device_get_device_type(device: *mut NMDevice) -> NMDeviceType;
}
