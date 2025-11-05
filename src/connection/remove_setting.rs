use crate::{NMConnection, raw::nm_connection_remove_setting, util::GetGType};

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSetting;
#[allow(unused_imports)]
use glib_2::gobject::GType;

impl NMConnection {
    /// Removes the [`NMSetting`] with the given [`GType`] from the [`NMConnection`]
    pub fn remove_setting<T: GetGType>(&self) {
        unsafe { nm_connection_remove_setting(self.handle(), T::get_gtype()) };
    }
}
