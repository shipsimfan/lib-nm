use crate::{NMClient, NMDeviceIter};
use glib_2::raw::glib::GPtrArray;
use std::ptr::null_mut;

impl<'client> NMDeviceIter<'client> {
    /// Create a new [`NMDeviceIter`] over `handle`
    pub unsafe fn new(handle: *const GPtrArray, client: &'client NMClient) -> Self {
        assert_ne!(handle, null_mut());
        let ptr_array = unsafe { &*handle };
        let slice = if ptr_array.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(ptr_array.data, ptr_array.len as _) }
        };
        let iter = slice.iter();
        NMDeviceIter { iter, client }
    }
}
