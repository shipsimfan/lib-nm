use crate::{
    NMConnection, NMSettingIP4Config, NMSettingIPConfig,
    raw::{self, nm_setting_ip4_config_new},
};

impl<'connection> NMSettingIP4Config<'connection> {
    /// Creates a new [`NMSettingIP4Config`] object with default values
    pub fn new() -> Self {
        let handle = unsafe { nm_setting_ip4_config_new() };
        unsafe { NMSettingIP4Config::new_raw(handle, None) }
    }

    /// Create a new [`NMSettingIP4Config`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMSettingIP4Config,
        connection: Option<&'connection NMConnection>,
    ) -> Self {
        let ip_config_settings = unsafe { NMSettingIPConfig::new_raw(handle, connection) };
        NMSettingIP4Config { ip_config_settings }
    }
}
