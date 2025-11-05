use crate::{NMConnection, NMSetting, raw};
use glib_2::gobject::GObject;

impl<'connection> NMSetting<'connection> {
    /// Create a new [`NMSetting`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMSetting,
        connection: Option<&'connection NMConnection>,
    ) -> Self {
        let object = unsafe { GObject::new_raw(handle, connection.is_none()) };
        NMSetting { object, connection }
    }
}
