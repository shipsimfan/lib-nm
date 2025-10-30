use crate::{NMDevice, NMDeviceIter};

impl<'client> Iterator for NMDeviceIter<'client> {
    type Item = NMDevice<'client>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|handle| unsafe { NMDevice::new_raw(*handle, self.client) })
    }
}
