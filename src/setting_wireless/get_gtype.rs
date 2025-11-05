use crate::{NMSettingWireless, raw::nm_setting_wireless_get_type, util::GetGType};
use glib_2::gobject::GType;

impl<'connection> GetGType for NMSettingWireless<'connection> {
    fn get_gtype() -> GType {
        unsafe { nm_setting_wireless_get_type() }
    }
}
