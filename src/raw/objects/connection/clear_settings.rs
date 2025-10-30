use crate::raw::NMConnection;

#[link(name = "nm")]
unsafe extern "C" {
    /// Deletes all of `connection`'s settings.
    ///
    /// # Parameters
    ///  * `connection` - a [`NMConnection`]
    pub fn nm_connection_clear_settings(connection: *mut NMConnection);
}
