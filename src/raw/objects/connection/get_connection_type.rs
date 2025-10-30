use crate::raw::NMConnection;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSettingConnection;

#[link(name = "nm")]
unsafe extern "C" {
    /// A shortcut to return the type from the connection's [`NMSettingConnection`].
    ///
    /// # Parameters
    ///  * `connection` - the [`NMConnection`]
    ///
    /// # Returns
    /// the type from the connection's 'connection' setting
    pub fn nm_connection_get_connection_type(connection: *mut NMConnection) -> *const c_char;
}
