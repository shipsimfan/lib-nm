use crate::{NMClient, NMDevice, raw};
use glib_2::util::NewRaw;
use std::ptr::null_mut;

impl<'client> NMDevice<'client> {
    /// Create a new [`NMDevice`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::NMDevice, client: Option<&'client NMClient>) -> Self {
        assert_ne!(handle, null_mut());
        NMDevice { handle, client }
    }
}

impl<'client> NewRaw for NMDevice<'client> {
    unsafe fn new_raw(handle: *mut glib_2::raw::gobject::GObject, owned: bool) -> Self {
        assert!(!owned);
        unsafe { NMDevice::new_raw(handle, None) }
    }
}
