use crate::{NMDevice, NMDeviceWifi};

impl<'client> NMDeviceWifi<'client> {
    /// Create a new [`NMDeviceWifi`] from `device` without checking the type
    pub unsafe fn new_raw(device: NMDevice<'client>) -> Self {
        NMDeviceWifi { device }
    }
}
