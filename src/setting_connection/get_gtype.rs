use crate::{NMSettingConnection, raw::nm_setting_connection_get_type, util::GetGType};
use glib_2::gobject::GType;

impl<'connection> GetGType for NMSettingConnection<'connection> {
    fn get_gtype() -> GType {
        unsafe { nm_setting_connection_get_type() }
    }
}
