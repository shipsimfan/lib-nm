use crate::raw::{NMIPAddress, NMSettingIPConfig};
use std::ffi::c_int;

#[link(name = "nm")]
unsafe extern "C" {
    /// # Parameters
    ///  * `setting` - the [`NMSettingIPConfig`]
    ///  * `idx` - index number of the address to return
    ///
    /// # Returns
    /// the address at index `idx`.
    pub fn nm_setting_ip_config_get_address(
        setting: *mut NMSettingIPConfig,
        idx: c_int,
    ) -> *mut NMIPAddress;
}
