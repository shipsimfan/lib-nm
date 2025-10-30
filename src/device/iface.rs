use crate::{NMDevice, raw::nm_device_get_iface};
use std::{borrow::Cow, ffi::CStr};

impl<'client> NMDevice<'client> {
    /// Gets the interface name of the [`NMDevice`]
    pub fn iface(&self) -> Cow<str> {
        self.iface_raw().to_string_lossy()
    }

    /// Gets the interface name of the [`NMDevice`] as a raw [`CStr`]
    pub fn iface_raw(&self) -> &CStr {
        unsafe { CStr::from_ptr(nm_device_get_iface(self.handle)) }
    }
}
