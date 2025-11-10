use crate::{NMIPAddress, raw::nm_ip_address_unref};

impl<'setting, 'connection> Drop for NMIPAddress<'setting, 'connection> {
    fn drop(&mut self) {
        if self.owned {
            unsafe { nm_ip_address_unref(self.handle) };
        }
    }
}
