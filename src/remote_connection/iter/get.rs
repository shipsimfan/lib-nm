use crate::NMRemoteConnectionIter;

impl<'owner, Owner> NMRemoteConnectionIter<'owner, Owner> {
    /// Get the owner of the connections
    pub fn owner(&self) -> &'owner Owner {
        self.owner
    }
}
