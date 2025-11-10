use crate::{
    NMRemoteConnection,
    raw::{nm_remote_connection_commit_changes_async, nm_remote_connection_commit_changes_finish},
};
use glib_2::{
    gio::{GAsyncReadyCallback, GAsyncResult, GCancellable},
    glib::GError,
    raw::glib::{FALSE, TRUE},
    util::CallbackData,
};
use std::ptr::null_mut;

impl<'owner, Owner> NMRemoteConnection<'owner, Owner> {
    /// Asynchronously sends any local changes to the settings and properties of this connection to
    /// NetworkManager.
    pub fn commit_changes_async<
        Callback: GAsyncReadyCallback<Object = NMRemoteConnection<'static>>,
    >(
        &self,
        save_to_disk: bool,
        cancellable: Option<&GCancellable>,
        user_data: Callback::UserData,
    ) {
        unsafe {
            nm_remote_connection_commit_changes_async(
                self.handle(),
                if save_to_disk { TRUE } else { FALSE },
                cancellable
                    .map(|cancellable| cancellable.handle())
                    .unwrap_or(null_mut()),
                Callback::trampoline,
                user_data.into_ptr(),
            );
        }
    }

    /// Gets the result of a call to [`NMRemoteConnection::commit_changes_async`].
    pub fn commit_changes_finish(&self, result: &GAsyncResult) -> Result<(), GError> {
        let mut error = null_mut();
        let result = unsafe {
            nm_remote_connection_commit_changes_finish(self.handle(), result.handle(), &mut error)
        };

        if result == FALSE {
            return Err(unsafe { GError::new_raw(error, true) });
        }

        Ok(())
    }
}
