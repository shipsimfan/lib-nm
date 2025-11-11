use crate::NMDevice;

impl<'client> NMDevice<'client> {
    /// Get this [`NMDevice`] without a client
    pub fn clone_static(&self) -> NMDevice<'static> {
        NMDevice {
            object: self.object.clone(),
            client: None,
        }
    }
}
