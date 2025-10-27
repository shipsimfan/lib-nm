use crate::NMActiveConnection;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::NMConnection;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the [`NMConnection`]'s type.
    ///
    /// # Parameters
    ///  * `connection` - a [`NMActiveConnection`]
    ///
    /// # Returns
    ///  the type of the [`NMConnection`] that backs the [`NMActiveConnection`]. This is the
    /// internal string used by the connection, and must not be modified.
    pub fn nm_active_connection_get_connection_type(
        connection: *mut NMActiveConnection,
    ) -> *const c_char;
}
