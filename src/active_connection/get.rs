use crate::NMActiveConnection;

impl<'owner, Owner> NMActiveConnection<'owner, Owner> {
    /// Get the owner of this connection
    pub fn owner(&self) -> Option<&'owner Owner> {
        self.owner
    }
}
