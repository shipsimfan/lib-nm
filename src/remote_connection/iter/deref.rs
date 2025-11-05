use crate::{NMRemoteConnectionIter, raw};
use glib_2::glib::GPtrArray;
use std::ops::Deref;

impl<'owner, Owner> Deref for NMRemoteConnectionIter<'owner, Owner> {
    type Target = GPtrArray<'owner, raw::NMRemoteConnection>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<'owner, Owner> AsRef<GPtrArray<'owner, raw::NMRemoteConnection>>
    for NMRemoteConnectionIter<'owner, Owner>
{
    fn as_ref(&self) -> &GPtrArray<'owner, raw::NMRemoteConnection> {
        &self.array
    }
}
