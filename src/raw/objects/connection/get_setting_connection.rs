use crate::raw::{NMConnection, NMSettingConnection};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// A shortcut to return any [`NMSettingConnection`] the `connection` might contain.
    ///
    /// # Parameters
    ///  * `connection` - the [`NMConnection`]
    ///
    /// # Returns
    /// an [`NMSettingConnection`] if the `connection` contains one, otherwise [`null_mut`].
    pub fn nm_connection_get_setting_connection(
        connection: *mut NMConnection,
    ) -> *mut NMSettingConnection;
}
