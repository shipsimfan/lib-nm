use crate::{NMIPAddress, NMIPAddressIter};

impl<'owner, Owner> Iterator for NMIPAddressIter<'owner, Owner> {
    type Item = NMIPAddress<'owner, Owner>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len() {
            return None;
        }

        let ret = unsafe { NMIPAddress::new_raw(self[self.index], Some(self.owner), false) };
        self.index += 1;
        Some(ret)
    }
}
