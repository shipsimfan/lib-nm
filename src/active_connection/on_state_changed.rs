use crate::{NMActiveConnection, raw};
use glib_2::{
    G_CALLBACK,
    gobject::GConnectFlags,
    raw::{
        glib::{guint, gulong},
        gobject::g_signal_connect_data,
    },
    util::CallbackData,
};

impl<'owner, Owner> NMActiveConnection<'owner, Owner> {
    /// Connects a [`NMActiveConnectionStateChangedCallback`] function to the "state-changed" signal
    pub fn on_state_changed<Callback: NMActiveConnectionStateChangedCallback>(
        &self,
        data: Callback::UserData,
        connect_flags: GConnectFlags,
    ) -> gulong {
        unsafe {
            g_signal_connect_data(
                self.handle(),
                c"state-changed".as_ptr(),
                G_CALLBACK!(Callback::trampoline),
                data.into_ptr(),
                Some(Callback::drop),
                connect_flags,
            )
        }
    }
}

/// A callback to data signal
pub trait NMActiveConnectionStateChangedCallback {
    /// The type of data to be passed to the callback
    type UserData: CallbackData;

    /// Called when the signal emits data
    fn callback(
        active_connection: NMActiveConnection,
        state: guint,
        reason: guint,
        data: &Self::UserData,
    );

    /// Converts a [`glib_2::raw::gobject::GCallback`] into a
    /// [`NMActiveConnectionStateChangedCallback`]
    extern "C" fn trampoline(
        active_connection: *mut raw::NMActiveConnection,
        state: guint,
        reason: guint,
        data: glib_2::raw::glib::gpointer,
    ) {
        let active_connection =
            unsafe { NMActiveConnection::new_raw(active_connection, None, false) };
        let data = unsafe { Self::UserData::from_ptr(data) };

        Self::callback(active_connection, state, reason, &data);

        // Convert the data back into a pointer to prevent dropping
        data.into_ptr();
    }

    /// Implements a [`raw::gobject::GClosureNotify`] for dropping the user data
    extern "C" fn drop(data: glib_2::raw::glib::gpointer, _: *mut glib_2::raw::gobject::GClosure) {
        drop(unsafe { Self::UserData::from_ptr(data) });
    }
}
