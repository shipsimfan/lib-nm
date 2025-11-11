use crate::{NMIPAddress, raw::nm_ip_address_unref};

impl<'owner, Owner> Drop for NMIPAddress<'owner, Owner> {
    fn drop(&mut self) {
        if self.owned {
            unsafe { nm_ip_address_unref(self.handle) };
        }
    }
}
