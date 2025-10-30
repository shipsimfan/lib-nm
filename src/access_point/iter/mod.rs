use crate::{NMDeviceWifi, raw};

mod iter;
mod new;

/// An iterator which produces [`crate::NMAccessPoint`]s
pub struct NMAccessPointIter<'client, 'device> {
    /// The underlying iterator
    inner: std::slice::Iter<'device, *mut raw::NMAccessPoint>,

    /// The device the access points belong to
    device: &'device NMDeviceWifi<'client>,
}
