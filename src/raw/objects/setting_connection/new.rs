use crate::raw::NMSetting;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::NMSettingConnection;

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMSettingConnection`] object with default values.
    ///
    /// # Returns
    /// the new empty [`NMSettingConnection`] object
    pub fn nm_setting_connection_new() -> *mut NMSetting;
}
