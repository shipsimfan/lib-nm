use crate::{
    NMClient, NMConnection, NMRemoteConnection,
    raw::{nm_client_add_connection_async, nm_client_add_connection_finish},
};
use glib_2::{
    gio::{GAsyncReadyCallback, GAsyncResult, GCancellable},
    glib::GError,
    raw::glib::{FALSE, TRUE},
    util::CallbackData,
};
use std::ptr::null_mut;

impl NMClient {
    /// Requests that the remote settings service add the given settings to a new connection
    pub fn add_connection_async<Callback: GAsyncReadyCallback<Object = Self>>(
        &self,
        connection: &NMConnection,
        save_to_disk: bool,
        cancellable: Option<&GCancellable>,
        user_data: Callback::UserData,
    ) {
        unsafe {
            nm_client_add_connection_async(
                self.handle(),
                connection.handle(),
                if save_to_disk { TRUE } else { FALSE },
                cancellable
                    .map(|cancellable| cancellable.handle())
                    .unwrap_or(null_mut()),
                Callback::trampoline,
                user_data.into_ptr(),
            );
        }
    }

    /// Gets the result of a call to [`NMClient::add_connection_async`]
    pub fn add_connection_finish<'client>(
        &'client self,
        result: &GAsyncResult,
    ) -> Result<NMRemoteConnection<'client, Self>, GError> {
        let mut error = null_mut();
        let handle =
            unsafe { nm_client_add_connection_finish(self.handle(), result.handle(), &mut error) };
        if handle == null_mut() {
            return Err(unsafe { GError::new_raw(error, true) });
        }

        Ok(unsafe { NMRemoteConnection::new_raw(handle, Some(self), false) })
    }
}
