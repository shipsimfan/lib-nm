use crate::{NMClient, NMDeviceIter, raw};

impl<'client> NMDeviceIter<'client> {
    /// Create a new [`NMDeviceIter`] over `inner`
    pub unsafe fn new(
        inner: std::slice::Iter<'client, *mut raw::NMDevice>,
        client: &'client NMClient,
    ) -> Self {
        NMDeviceIter { inner, client }
    }
}
