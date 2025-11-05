use crate::{NMAccessPointIter, NMDeviceWifi};
use glib_2::raw::glib::GPtrArray;
use std::ptr::null_mut;

impl<'client, 'device> NMAccessPointIter<'client, 'device> {
    /// Create a new [`NMAccessPointIter`] over `handle`
    pub unsafe fn new(handle: *const GPtrArray, device: &'device NMDeviceWifi<'client>) -> Self {
        assert_ne!(handle, null_mut());
        let ptr_array = unsafe { &*handle };
        let slice = if ptr_array.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(ptr_array.data, ptr_array.len as _) }
        };
        let iter = slice.iter();
        NMAccessPointIter { iter, device }
    }
}
