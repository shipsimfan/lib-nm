use crate::NMActiveConnection;

impl<'owner, Owner> NMActiveConnection<'owner, Owner> {
    /// Clone this connection to a static lifetime
    pub fn clone_static(&self) -> NMActiveConnection<'static> {
        NMActiveConnection {
            object: self.object.clone(),
            owner: None,
        }
    }
}
