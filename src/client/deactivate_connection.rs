use crate::{
    NMActiveConnection, NMClient,
    raw::{nm_client_deactivate_connection_async, nm_client_deactivate_connection_finish},
};
use glib_2::{
    gio::{GAsyncReadyCallback, GAsyncResult, GCancellable},
    glib::GError,
    raw::glib::FALSE,
    util::CallbackData,
};
use std::ptr::null_mut;

impl NMClient {
    /// Asynchronously deactivates an active [`NMActiveConnection`]
    pub fn deactivate_connection_async<Callback: GAsyncReadyCallback<Object = Self>, Owner>(
        &self,
        active: &NMActiveConnection<Owner>,
        cancellable: Option<&GCancellable>,
        user_data: Callback::UserData,
    ) {
        unsafe {
            nm_client_deactivate_connection_async(
                self.handle(),
                active.handle(),
                cancellable
                    .map(|cancellable| cancellable.handle())
                    .unwrap_or(null_mut()),
                Callback::trampoline,
                user_data.into_ptr(),
            )
        };
    }

    /// Gets the result of a call to [`NMClient::deactivate_connection_async`]
    pub fn deactivate_connection_finish<'client>(
        &'client self,
        result: &GAsyncResult,
    ) -> Result<(), GError> {
        let mut error = null_mut();
        let result = unsafe {
            nm_client_deactivate_connection_finish(self.handle(), result.handle(), &mut error)
        };
        if result == FALSE {
            return Err(unsafe { GError::new_raw(error, true) });
        }

        Ok(())
    }
}
