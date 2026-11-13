use crate::{NMSettingWired, raw::nm_setting_wired_get_type, util::GetGType};
use glib_2::gobject::GType;

impl<'connection> GetGType for NMSettingWired<'connection> {
    fn get_gtype() -> GType {
        unsafe { nm_setting_wired_get_type() }
    }
}
