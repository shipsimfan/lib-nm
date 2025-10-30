use crate::{NMClient, raw::nm_client_new};
use glib_2::{gio::GCancellable, glib::GError};
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
            return Err(unsafe { GError::new_raw(error) });
        }

        Ok(NMClient { handle })
    }
}
