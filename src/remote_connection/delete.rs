use crate::{
    NMRemoteConnection,
    raw::{nm_remote_connection_delete_async, nm_remote_connection_delete_finish},
};
use glib_2::{
    gio::{GAsyncReadyCallback, GAsyncResult, GCancellable},
    glib::GError,
    raw::glib::FALSE,
    util::CallbackData,
};
use std::ptr::null_mut;

impl<'owner, Owner> NMRemoteConnection<'owner, Owner> {
    /// Asynchronously deletes the connection.
    pub fn delete_async<Callback: GAsyncReadyCallback<Object = NMRemoteConnection<'static>>>(
        &self,
        cancellable: Option<&GCancellable>,
        user_data: Callback::UserData,
    ) {
        unsafe {
            nm_remote_connection_delete_async(
                self.handle(),
                cancellable
                    .map(|cancellable| cancellable.handle())
                    .unwrap_or(null_mut()),
                Callback::trampoline,
                user_data.into_ptr(),
            );
        }
    }

    /// Gets the result of a call to [`NMRemoteConnection::delete_async`].
    pub fn delete_finish(&self, result: &GAsyncResult) -> Result<(), GError> {
        let mut error = null_mut();
        let result = unsafe {
            nm_remote_connection_delete_finish(self.handle(), result.handle(), &mut error)
        };

        if result == FALSE {
            return Err(unsafe { GError::new_raw(error, true) });
        }

        Ok(())
    }
}
