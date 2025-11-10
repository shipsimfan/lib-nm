use crate::{NMActiveConnection, NMRemoteConnection, raw::nm_active_connection_get_connection};

impl<'owner, Owner> NMActiveConnection<'owner, Owner> {
    /// Gets the [`NMRemoteConnection`] associated with this connection
    pub fn connection(&self) -> NMRemoteConnection<'owner, Owner> {
        let handle = unsafe { nm_active_connection_get_connection(self.handle()) };
        unsafe { NMRemoteConnection::new_raw(handle, self.owner, false) }
    }
}
