use crate::raw::{NMConnection, NMSettingIPConfig};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSettingIP4Config;

#[link(name = "nm")]
unsafe extern "C" {
    /// A shortcut to return any [`NMSettingIP4Config`] the connection might contain.
    ///
    /// Note that it returns the value as type [`NMSettingIPConfig`], since the vast majority of
    /// IPv4-setting-related methods are on that type, not [`NMSettingIP4Config`].
    pub fn nm_connection_get_setting_ip4_config(
        connection: *mut NMConnection,
    ) -> *mut NMSettingIPConfig;
}
