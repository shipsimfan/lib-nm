use crate::{NMIPAddressIter, raw};
use glib_2::glib::GPtrArray;
use std::ops::Deref;

impl<'owner, Owner> Deref for NMIPAddressIter<'owner, Owner> {
    type Target = GPtrArray<'owner, raw::NMIPAddress>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<'owner, Owner> AsRef<GPtrArray<'owner, raw::NMIPAddress>> for NMIPAddressIter<'owner, Owner> {
    fn as_ref(&self) -> &GPtrArray<'owner, raw::NMIPAddress> {
        &self.array
    }
}
