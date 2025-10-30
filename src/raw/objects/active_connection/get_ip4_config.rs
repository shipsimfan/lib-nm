use crate::raw::{NMActiveConnection, NMIPConfig};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the current IPv4 NMIPConfig associated with the [`NMActiveConnection`].
    ///
    /// # Parameters
    ///  * `connection` - an [`NMActiveConnection`]
    ///
    /// # Returns
    /// the IPv4 [`NMIPConfig`], or [`null_mut`] if the connection is not in the
    /// [`NM_ACTIVE_CONNECTION_STATE_ACTIVATED`] state.
    pub fn nm_active_connection_get_ip4_config(
        connection: *mut NMActiveConnection,
    ) -> *mut NMIPConfig;
}
