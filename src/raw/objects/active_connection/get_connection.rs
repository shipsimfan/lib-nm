use crate::raw::{NMActiveConnection, NMRemoteConnection};

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the [`NMRemoteConnection`] associated with connection .
    ///
    /// # Parameters
    ///  * `connection` - a [`NMActiveConnection`]
    ///
    /// # Returns
    /// the [`NMRemoteConnection`] which this [`NMActiveConnection`] is an active instance of.
    pub fn nm_active_connection_get_connection(
        connection: *mut NMActiveConnection,
    ) -> *mut NMRemoteConnection;
}
