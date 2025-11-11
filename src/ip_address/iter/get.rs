use crate::NMIPAddressIter;

impl<'owner, Owner> NMIPAddressIter<'owner, Owner> {
    /// Get the owner of the addresses
    pub fn owner(&self) -> &'owner Owner {
        self.owner
    }
}
