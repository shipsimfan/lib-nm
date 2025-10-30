use crate::{NMAccessPointIter, NMDeviceWifi, raw};

impl<'client, 'device> NMAccessPointIter<'client, 'device> {
    /// Create a new [`NMAccessPointIter`] over `inner`
    pub unsafe fn new(
        inner: std::slice::Iter<'device, *mut raw::NMAccessPoint>,
        device: &'device NMDeviceWifi<'client>,
    ) -> Self {
        NMAccessPointIter { inner, device }
    }
}
