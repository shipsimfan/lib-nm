use crate::NMClient;
use glib_2::glib::GPtrArray;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `client` - the [`NMClient`]
    ///
    /// # Returns
    /// an array containing all connections provided by the remote settings service. The returned
    /// array is owned by the [`NMClient`] object and should not be modified.
    ///
    /// The connections are as received from D-Bus and might not validate according to
    /// [`nm_connection_verify`].
    pub fn nm_client_get_connections(client: *mut NMClient) -> *const GPtrArray;
}
