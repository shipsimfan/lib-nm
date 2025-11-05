use crate::{NMActiveConnection, raw};
use glib_2::{gobject::GObject, util::NewRaw};

impl<'owner, Owner> NMActiveConnection<'owner, Owner> {
    /// Create a new [`NMActiveConnection`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMActiveConnection,
        owner: Option<&'owner Owner>,
        owned: bool,
    ) -> Self {
        let object = unsafe { GObject::new_raw(handle, owned) };
        NMActiveConnection { object, owner }
    }
}

impl<'owner, Owner> NewRaw for NMActiveConnection<'owner, Owner> {
    unsafe fn new_raw(handle: *mut glib_2::raw::gobject::GObject, owned: bool) -> Self {
        unsafe { NMActiveConnection::new_raw(handle, None, owned) }
    }
}
