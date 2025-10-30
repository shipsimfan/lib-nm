use crate::{NMDevice, NMDeviceWifi};
use std::ops::Deref;

impl<'client> Deref for NMDeviceWifi<'client> {
    type Target = NMDevice<'client>;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}
