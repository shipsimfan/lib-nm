use crate::{NMDevice, raw::nm_device_get_hw_address};
use std::{borrow::Cow, ffi::CStr};

impl<'client> NMDevice<'client> {
    /// Gets the hardware address of the [`NMDevice`]
    pub fn hw_address<'a>(&'a self) -> Cow<'a, str> {
        self.hw_address_raw().to_string_lossy()
    }

    /// Gets the hardware address of the [`NMDevice`] as a raw [`CStr`]
    pub fn hw_address_raw(&self) -> &CStr {
        unsafe { CStr::from_ptr(nm_device_get_hw_address(self.handle())) }
    }
}
