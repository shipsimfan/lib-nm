use crate::raw::{NMConnection, NMSettingWireless};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// A shortcut to return any [`NMSettingWireless`] the `connection` might contain.
    ///
    /// # Parameters
    ///  * `connection` - the [`NMConnection`]
    ///
    /// # Returns
    /// an [`NMSettingWireless`] if the `connection` contains one, otherwise [`null_mut`].
    pub fn nm_connection_get_setting_wireless(
        connection: *mut NMConnection,
    ) -> *mut NMSettingWireless;
}
