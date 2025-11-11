use crate::NMIPAddressIter;
use glib_2::glib::GPtrArray;

impl<'owner, Owner> NMIPAddressIter<'owner, Owner> {
    /// Create a new [`NMIPAddressIter`] over `handle`
    pub unsafe fn new(
        handle: *const glib_2::raw::glib::GPtrArray,
        owner: &'owner Owner,
        owned: bool,
    ) -> Self {
        let array = unsafe { GPtrArray::new_raw(handle.cast_mut(), owned) };

        NMIPAddressIter {
            array,
            index: 0,
            owner,
        }
    }
}
