use crate::NMDeviceWifi;

impl<'client> NMDeviceWifi<'client> {
    /// Get this [`NMDeviceWifi`] without a client
    pub fn clone_static(&self) -> NMDeviceWifi<'static> {
        unsafe { NMDeviceWifi::new_raw(self.device.clone_static()) }
    }
}
