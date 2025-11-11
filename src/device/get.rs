use crate::{NMClient, NMDevice};

impl<'client> NMDevice<'client> {
    /// Get the [`NMClient`] this device belongs to
    pub fn client(&self) -> Option<&'client NMClient> {
        self.client
    }
}
