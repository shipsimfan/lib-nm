use crate::raw::{NMConnection, NMSetting};

#[link(name = "nm")]
unsafe extern "C" {
    /// Adds a [`NMSetting`] to the connection, replacing any previous [`NMSetting`] of the same
    /// name which has previously been added to the [`NMConnection`]. The connection takes
    /// ownership of the [`NMSetting`] object and does not increase the setting object's reference
    /// count.
    ///
    /// # Parameters
    ///  * `connection` - a [`NMConnection`]
    ///  * `setting` - the [`NMSetting`] to add to the connection object.
    pub fn nm_connection_add_setting(connection: *mut NMConnection, setting: *mut NMSetting);
}
