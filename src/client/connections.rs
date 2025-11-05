use crate::{NMClient, NMRemoteConnectionIter, raw::nm_client_get_connections};

impl NMClient {
    /// Gets all connections available on the client
    pub fn connections(&self) -> NMRemoteConnectionIter<Self> {
        let handle = unsafe { nm_client_get_connections(self.handle()) };
        unsafe { NMRemoteConnectionIter::new(handle, self, false) }
    }
}
