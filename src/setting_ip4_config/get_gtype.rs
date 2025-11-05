use crate::{NMSettingIP4Config, raw::nm_setting_ip4_config_get_type, util::GetGType};
use glib_2::gobject::GType;

impl<'connection> GetGType for NMSettingIP4Config<'connection> {
    fn get_gtype() -> GType {
        unsafe { nm_setting_ip4_config_get_type() }
    }
}
