use crate::{NMRemoteConnection, NMRemoteConnectionIter};

impl<'owner, Owner> Iterator for NMRemoteConnectionIter<'owner, Owner> {
    type Item = NMRemoteConnection<'owner, Owner>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len() {
            return None;
        }

        let ret = unsafe { NMRemoteConnection::new_raw(self[self.index], Some(self.owner), false) };
        self.index += 1;
        Some(ret)
    }
}
