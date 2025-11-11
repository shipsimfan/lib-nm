use crate::{NMDevice, NMDeviceIter};

impl<'client> Iterator for NMDeviceIter<'client> {
    type Item = NMDevice<'client>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|handle| unsafe { NMDevice::new_raw(*handle, Some(self.client), false) })
    }
}
