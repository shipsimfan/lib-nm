use crate::{NMClient, raw};
use glib_2::glib::GPtrArray;

mod deref;
mod get;
mod iter;
mod new;

/// An iterator which produces [`crate::NMRemoteConnection`]s
pub struct NMRemoteConnectionIter<'owner, Owner = NMClient> {
    /// A handle to the raw underlying array
    array: GPtrArray<'owner, raw::NMRemoteConnection>,

    /// The underlying iterator
    index: usize,

    /// The owner of the connections
    owner: &'owner Owner,
}
