use crate::raw::NMClient;
use glib_2::glib::GPtrArray;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::nm_connection_verify;

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
