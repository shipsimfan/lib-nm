use glib_2::raw::glib::FALSE;

use crate::{
    NMIPAddress, NMSettingIPConfig,
    raw::{
        nm_setting_ip_config_add_address, nm_setting_ip_config_get_address,
        nm_setting_ip_config_get_num_addresses,
    },
};
use std::ptr::null_mut;

impl<'connection> NMSettingIPConfig<'connection> {
    /// Get the number of configured addresses
    pub fn num_addresses(&self) -> glib_2::raw::glib::guint {
        unsafe { nm_setting_ip_config_get_num_addresses(self.handle()) }
    }

    /// Get the address at index `idx`
    pub fn address(&self, idx: glib_2::raw::glib::guint) -> Option<NMIPAddress> {
        let handle = unsafe { nm_setting_ip_config_get_address(self.handle(), idx as _) };
        if handle == null_mut() {
            return None;
        }

        Some(unsafe { NMIPAddress::new_raw(handle, Some(self), false) })
    }

    /// Adds a new IP address and associated information to the setting
    pub fn add_address(&self, address: &NMIPAddress) -> bool {
        let result = unsafe { nm_setting_ip_config_add_address(self.handle(), address.handle()) };
        result != FALSE
    }
}
