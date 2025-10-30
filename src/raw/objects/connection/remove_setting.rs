use crate::raw::NMConnection;
use glib_2::raw::gobject::GType;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSetting;

#[link(name = "nm")]
unsafe extern "C" {
    /// Removes the [`NMSetting`] with the given [`GType`] from the [`NMConnection`]. This
    /// operation dereferences the [`NMSetting`] object.
    ///
    /// # Parameters
    ///  * `connection` - a [`NMConnection`]
    ///  * `setting_type` - the [`GType`] of the setting object to remove
    pub fn nm_connection_remove_setting(connection: *mut NMConnection, setting_type: GType);
}
