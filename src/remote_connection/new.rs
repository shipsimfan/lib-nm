use crate::{NMConnection, NMRemoteConnection, raw};
use glib_2::util::NewRaw;

impl<'owner, Owner> NMRemoteConnection<'owner, Owner> {
    /// Create a new [`NMRemoteConnection`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMRemoteConnection,
        owner: Option<&'owner Owner>,
        owned: bool,
    ) -> Self {
        let connection = unsafe { NMConnection::new_raw(handle, owned) };
        NMRemoteConnection { connection, owner }
    }
}

impl<'owner, Owner> NewRaw for NMRemoteConnection<'owner, Owner> {
    unsafe fn new_raw(handle: *mut glib_2::raw::gobject::GObject, owned: bool) -> Self {
        unsafe { NMRemoteConnection::new_raw(handle, None, owned) }
    }
}
