use crate::{NMAccessPoint, NMDeviceWifi, raw};
use std::ptr::null_mut;

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Create a new [`NMAccessPoint`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMAccessPoint,
        device: &'device NMDeviceWifi<'client>,
    ) -> Self {
        assert_ne!(handle, null_mut());
        NMAccessPoint { handle, device }
    }
}
