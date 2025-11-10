use crate::NMDevice;

impl<'client> NMDevice<'client> {
    /// Get this [`NMDevice`] without a client
    pub unsafe fn drop_client(&self) -> NMDevice<'static> {
        NMDevice {
            handle: self.handle,
            client: None,
        }
    }
}
