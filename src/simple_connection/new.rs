use crate::{NMConnection, NMSimpleConnection, raw::nm_simple_connection_new};

impl NMSimpleConnection {
    /// Creates a new [`NMSimpleConnection`] object with no [`crate::NMSetting`] objects.
    pub fn new() -> Self {
        let handle = unsafe { nm_simple_connection_new() };
        let connection = unsafe { NMConnection::new_raw(handle, true) };
        NMSimpleConnection { connection }
    }
}
