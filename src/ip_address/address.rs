use crate::{NMIPAddress, raw::nm_ip_address_get_address};
use std::{borrow::Cow, ffi::CStr};

impl<'owner, Owner> NMIPAddress<'owner, Owner> {
    /// Gets the IP address property of this address object
    pub fn address(&self) -> Cow<str> {
        self.address_raw().to_string_lossy()
    }

    /// Gets the IP address property of this address object as a raw [`CStr`]
    pub fn address_raw(&self) -> &CStr {
        unsafe { CStr::from_ptr(nm_ip_address_get_address(self.handle)) }
    }
}
