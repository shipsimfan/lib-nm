use crate::{
    NMIPAddress, NMSetting,
    raw::{self, nm_ip_address_new_binary},
};
use glib_2::glib::GError;
use linux::sys::socket::{AF_INET, AF_INET6};
use std::{net::IpAddr, ptr::null_mut};

impl<'setting, 'connection> NMIPAddress<'setting, 'connection> {
    /// Creates a new [`NMIPAddress`] object
    pub fn new_binary(address: IpAddr, prefix: glib_2::raw::glib::guint) -> Result<Self, GError> {
        let mut error = null_mut();
        let handle = match address {
            IpAddr::V4(addr) => {
                let addr = addr.octets();
                unsafe {
                    nm_ip_address_new_binary(AF_INET, addr.as_ptr().cast(), prefix as _, &mut error)
                }
            }
            IpAddr::V6(addr) => {
                let addr = addr.octets();
                unsafe {
                    nm_ip_address_new_binary(
                        AF_INET6,
                        addr.as_ptr().cast(),
                        prefix as _,
                        &mut error,
                    )
                }
            }
        };

        if handle == null_mut() {
            return Err(unsafe { GError::new_raw(error, true) });
        }

        Ok(unsafe { NMIPAddress::new_raw(handle, None, true) })
    }

    /// Create a new [`NMIPAddress`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMIPAddress,
        setting: Option<&'setting NMSetting<'connection>>,
        owned: bool,
    ) -> Self {
        assert_ne!(handle, null_mut());
        NMIPAddress {
            handle,
            setting,
            owned,
        }
    }
}
