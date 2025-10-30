use crate::raw::{NMIPAddress, NMSettingIPConfig};
use glib_2::raw::glib::gboolean;

// rustdoc imports
#[allow(unused_imports)]
use glib_2::raw::glib::{FALSE, TRUE};

#[link(name = "nm")]
unsafe extern "C" {
    /// Adds a new IP address and associated information to the setting. The given address is
    /// duplicated internally and is not changed by this function.
    ///
    /// # Parameters
    ///  * `setting` - the [`NMSettingIPConfig`]
    ///  * `address` - the new address to add
    ///
    /// # Returns
    /// [`TRUE`] if the address was added; [`FALSE`] if the address was already known.
    pub fn nm_setting_ip_config_add_address(
        setting: *mut NMSettingIPConfig,
        address: *mut NMIPAddress,
    ) -> gboolean;
}
