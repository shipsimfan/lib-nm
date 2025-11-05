use crate::{
    NMClient,
    raw::{self, nm_client_new},
};
use glib_2::{gio::GCancellable, glib::GError, gobject::GObject, util::NewRaw};
use std::ptr::null_mut;

impl NMClient {
    /// Creates a new [`NMClient`] synchronously.
    pub fn new(cancellable: Option<&mut GCancellable>) -> Result<Self, GError> {
        let mut error = null_mut();
        let cancellable = cancellable
            .map(|cancellable| unsafe { cancellable.handle() })
            .unwrap_or(null_mut());

        let handle = unsafe { nm_client_new(cancellable, &mut error) };
        if handle == null_mut() {
            return Err(unsafe { GError::new_raw(error, true) });
        }

        Ok(unsafe { NMClient::new_raw(handle, true) })
    }

    /// Create a new [`NMClient`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::NMClient, owned: bool) -> Self {
        let object = unsafe { GObject::new_raw(handle, owned) };
        NMClient { object }
    }
}

impl NewRaw for NMClient {
    unsafe fn new_raw(handle: *mut glib_2::raw::gobject::GObject, owned: bool) -> Self {
        unsafe { NMClient::new_raw(handle, owned) }
    }
}
