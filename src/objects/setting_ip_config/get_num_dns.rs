use crate::NMSettingIPConfig;
use glib_2::glib::guint;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingIPConfig`]
    ///
    /// # Returns
    /// the number of configured DNS servers
    pub fn nm_setting_ip_config_get_num_dns(setting: *mut NMSettingIPConfig) -> guint;
}
