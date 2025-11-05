use crate::{NMConnection, raw};
use glib_2::{gobject::GObject, util::NewRaw};

impl NMConnection {
    /// Create a new [`NMConnection`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::NMConnection, owned: bool) -> Self {
        let object = unsafe { GObject::new_raw(handle, owned) };
        NMConnection { object }
    }
}

impl NewRaw for NMConnection {
    unsafe fn new_raw(handle: *mut glib_2::raw::gobject::GObject, owned: bool) -> Self {
        unsafe { NMConnection::new_raw(handle, owned) }
    }
}
