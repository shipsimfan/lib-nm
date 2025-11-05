use crate::{NMSettingIPConfig, raw::nm_setting_ip_config_get_type, util::GetGType};
use glib_2::gobject::GType;

impl<'connection> GetGType for NMSettingIPConfig<'connection> {
    fn get_gtype() -> GType {
        unsafe { nm_setting_ip_config_get_type() }
    }
}
