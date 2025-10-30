use crate::{NMClient, NMDevice, raw};
use std::ptr::null_mut;

impl<'client> NMDevice<'client> {
    /// Create a new [`NMDevice`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::NMDevice, client: &'client NMClient) -> Self {
        assert_ne!(handle, null_mut());
        NMDevice { handle, client }
    }
}
