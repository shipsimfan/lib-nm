use crate::NMRemoteConnectionIter;
use glib_2::glib::GPtrArray;

impl<'owner, Owner> NMRemoteConnectionIter<'owner, Owner> {
    /// Create a new [`NMRemoteConnectionIter`] over `handle`
    pub unsafe fn new(
        handle: *const glib_2::raw::glib::GPtrArray,
        owner: &'owner Owner,
        owned: bool,
    ) -> Self {
        let array = unsafe { GPtrArray::new_raw(handle.cast_mut(), owned) };

        NMRemoteConnectionIter {
            array,
            index: 0,
            owner,
        }
    }
}
