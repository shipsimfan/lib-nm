use crate::NMSettingIPConfig;
use glib_2::glib::gboolean;
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::nm_connection_verify;
#[allow(unused_imports)]
use glib_2::glib::{FALSE, TRUE};

#[link(name = "nm")]
unsafe extern "C" {
    /// Adds a new DNS server to the setting.
    ///
    /// # Parameters
    ///  * `setting` - the [`NMSettingIPConfig`]
    ///  * `dns` - the IP address of the DNS server to add
    ///
    /// # Returns
    /// [`TRUE`] if the DNS server was added; [`FALSE`] if the server was already known
    ///
    /// Before 1.42, setting `dns` to an invalid string was treated as user-error. Now, also
    /// invalid DNS values can be set, but will be rejected later during [`nm_connection_verify`].
    pub fn nm_setting_ip_config_add_dns(
        setting: *mut NMSettingIPConfig,
        dns: *const c_char,
    ) -> gboolean;
}
