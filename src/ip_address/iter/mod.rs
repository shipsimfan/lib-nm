use crate::raw;
use glib_2::glib::GPtrArray;

mod deref;
mod get;
mod iter;
mod new;

/// An iterator which produces [`crate::NMIPAddress`]s
pub struct NMIPAddressIter<'owner, Owner> {
    /// A handle to the raw underlying array
    array: GPtrArray<'owner, raw::NMIPAddress>,

    /// The underlying iterator
    index: usize,

    /// The owner of the connections
    owner: &'owner Owner,
}
