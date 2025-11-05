use crate::{
    NMActiveConnection, NMClient, NMConnection, NMDevice,
    raw::{nm_client_activate_connection_async, nm_client_activate_connection_finish},
};
use glib_2::{
    gio::{GAsyncReadyCallback, GAsyncResult, GCancellable},
    glib::GError,
    util::CallbackData,
};
use std::{ffi::CStr, ptr::null_mut};

impl NMClient {
    /// Asynchronously starts a connection to a particular network using the configuration settings
    /// from `connection` and the network device `device`.
    pub fn activate_connection_async<Callback: GAsyncReadyCallback<Object = Self>>(
        &self,
        connection: Option<&NMConnection>,
        device: Option<&NMDevice>,
        specific_object: Option<&CStr>,
        cancellable: Option<&GCancellable>,
        user_data: Callback::UserData,
    ) {
        unsafe {
            nm_client_activate_connection_async(
                self.handle(),
                connection
                    .map(|connection| connection.handle())
                    .unwrap_or(null_mut()),
                device.map(|device| device.handle()).unwrap_or(null_mut()),
                specific_object.map(CStr::as_ptr).unwrap_or(null_mut()),
                cancellable
                    .map(|cancellable| cancellable.handle())
                    .unwrap_or(null_mut()),
                Callback::trampoline,
                user_data.into_ptr(),
            )
        };
    }

    /// Gets the result of a call to [`NMClient::activate_connection_async`]
    pub fn activate_connection_finish<'client>(
        &'client self,
        result: &GAsyncResult,
    ) -> Result<NMActiveConnection<'client, Self>, GError> {
        let mut error = null_mut();
        let handle = unsafe {
            nm_client_activate_connection_finish(self.handle(), result.handle(), &mut error)
        };
        if handle == null_mut() {
            return Err(unsafe { GError::new_raw(error, true) });
        }

        Ok(unsafe { NMActiveConnection::new_raw(handle, Some(self), false) })
    }
}
