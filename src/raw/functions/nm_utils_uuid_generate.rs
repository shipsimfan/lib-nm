use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSettingConnection;
#[allow(unused_imports)]
use glib_2::raw::glib::g_free;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Returns
    ///  a newly allocated UUID suitable for use as the [`NMSettingConnection`] object's “id”:
    /// property. Should be freed with [`g_free`]
    pub fn nm_utils_uuid_generate() -> *mut c_char;
}
