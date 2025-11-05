use crate::NMRemoteConnection;

impl<'owner, Owner> NMRemoteConnection<'owner, Owner> {
    /// Get the owner of this connection
    pub fn owner(&self) -> Option<&'owner Owner> {
        self.owner
    }
}
