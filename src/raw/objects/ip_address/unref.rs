use crate::raw::NMIPAddress;

#[link(name = "nm")]
unsafe extern "C" {
    /// Decreases the reference count of the object. If the reference count reaches zero, the
    /// object will be destroyed.
    ///
    /// # Parameters
    ///  * `address` - the [`NMIPAddress`]
    pub fn nm_ip_address_unref(address: *mut NMIPAddress);
}
