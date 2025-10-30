use crate::raw::NMIPRoute;

#[link(name = "nm")]
unsafe extern "C" {
    /// Decreases the reference count of the object. If the reference count reaches zero, the
    /// object will be destroyed.
    ///
    /// # Parameters
    ///  * `route` - the [`NMIPRoute`]
    pub fn nm_ip_route_unref(route: *mut NMIPRoute);
}
