use crate::raw::NMConnection;
use glib_2::glib::{GError, gboolean};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMSettingWired, NMSettingWireless, NMSettingWirelessSecurity};
#[allow(unused_imports)]
use glib_2::glib::{FALSE, TRUE};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "nm")]
unsafe extern "C" {
    /// Validates the connection and all its settings. Each setting's properties have allowed
    /// values, and some values are dependent on other values. For example, if a Wi-Fi connection
    /// is security enabled, the [`NMSettingWireless`] setting object's 'security' property must
    /// contain the setting name of the [`NMSettingWirelessSecurity`] object, which must also be
    /// present in the connection for the connection to be valid. As another example, the
    /// [`NMSettingWired`] object's 'mac-address' property must be a validly formatted MAC address.
    /// The returned [`GError`] contains information about which setting and which property failed
    /// validation, and how it failed validation.
    ///
    /// # Parameters
    ///  * `connection` - the [`NMConnection`] to verify
    ///  * `error` - location to store error, or [`null_mut`]
    ///
    /// # Returns
    /// [`TRUE`] if the connection is valid, [`FALSE`] if it is not
    pub fn nm_connection_verify(connection: *mut NMConnection, error: *mut *mut GError)
    -> gboolean;
}
