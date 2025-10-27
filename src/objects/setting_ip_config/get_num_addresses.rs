use crate::NMSettingIPConfig;
use glib_2::glib::guint;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingIPConfig`]
    ///
    /// # Returns
    /// the number of configured addresses
    pub fn nm_setting_ip_config_get_num_addresses(settings: *mut NMSettingIPConfig) -> guint;
}
