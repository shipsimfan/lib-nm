use crate::{NMDevice, NMDeviceType, NMDeviceWifi, raw::nm_device_get_device_type};

impl<'client> NMDevice<'client> {
    /// Get the type of device this is
    pub fn r#type(&self) -> NMDeviceType {
        unsafe { nm_device_get_device_type(self.handle()) }
    }

    /// Get a view of this device for Wi-Fi, if it is a Wi-Fi device
    pub fn as_wifi(&self) -> Option<NMDeviceWifi<'client>> {
        match self.r#type() {
            NMDeviceType::Wifi => Some(unsafe { NMDeviceWifi::new_raw(self.clone()) }),
            _ => None,
        }
    }
}
