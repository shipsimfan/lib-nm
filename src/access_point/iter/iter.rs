use crate::{NMAccessPoint, NMAccessPointIter};

impl<'client, 'device> Iterator for NMAccessPointIter<'client, 'device> {
    type Item = NMAccessPoint<'client, 'device>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|handle| unsafe { NMAccessPoint::new_raw(*handle, self.device) })
    }
}
