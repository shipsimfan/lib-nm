use crate::raw::NMClient;
use glib_2::glib::GPtrArray;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMActiveConnection;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the active connections.
    ///
    /// # Parameters
    ///  * `client` - a [`NMClient`]
    ///
    /// # Returns
    /// a [`GPtrArray`] containing all the active [`NMActiveConnection`]s. The returned array is
    /// owned by the client and should not be modified.
    pub fn nm_client_get_active_connections(client: *mut NMClient) -> *const GPtrArray;
}
