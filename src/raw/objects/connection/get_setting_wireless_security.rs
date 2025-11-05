use crate::raw::{NMConnection, NMSettingWirelessSecurity};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// A shortcut to return any [`NMSettingWirelessSecurity`] the `connection` might contain.
    ///
    /// # Parameters
    ///  * `connection` - the [`NMConnection`]
    ///
    /// # Returns
    /// an [`NMSettingWirelessSecurity`] if the `connection` contains one, otherwise [`null_mut`].
    pub fn nm_connection_get_setting_wireless_security(
        connection: *mut NMConnection,
    ) -> *mut NMSettingWirelessSecurity;
}
