use crate::NMConnection;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSettingConnection;

#[link(name = "nm")]
unsafe extern "C" {
    /// A shortcut to return the ID from the connection's [`NMSettingConnection`].
    ///
    /// # Parameters
    ///  * `connection` - the [`NMConnection`]
    ///
    /// # Returns
    ///  the ID from the connection's 'connection' setting
    pub fn nm_connection_get_id(connection: *mut NMConnection) -> *const c_char;
}
