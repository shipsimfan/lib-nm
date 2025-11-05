use crate::{NMSettingIP6Config, raw::nm_setting_ip6_config_get_type, util::GetGType};
use glib_2::gobject::GType;

impl<'connection> GetGType for NMSettingIP6Config<'connection> {
    fn get_gtype() -> GType {
        unsafe { nm_setting_ip6_config_get_type() }
    }
}
