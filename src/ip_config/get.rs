use crate::NMIPConfig;

impl<'owner, Owner> NMIPConfig<'owner, Owner> {
    /// Get the owner of this IP configuration
    pub fn owner(&self) -> Option<&'owner Owner> {
        self.owner
    }
}
