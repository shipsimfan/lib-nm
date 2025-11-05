use crate::{NMClient, raw};

mod iter;
mod new;

/// An iterator which produces [`crate::NMDevice`]s
pub struct NMDeviceIter<'client> {
    /// The underlying iterator
    iter: std::slice::Iter<'client, *mut raw::NMDevice>,

    /// The client the devices belong to
    client: &'client NMClient,
}
