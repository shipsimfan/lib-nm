use crate::{
    NMSettingWirelessSecurity, raw::nm_setting_wireless_security_get_type, util::GetGType,
};
use glib_2::gobject::GType;

impl<'connection> GetGType for NMSettingWirelessSecurity<'connection> {
    fn get_gtype() -> GType {
        unsafe { nm_setting_wireless_security_get_type() }
    }
}
