use crate::NMRemoteConnection;

impl<'owner, Owner> NMRemoteConnection<'owner, Owner> {
    /// Clone this connection to a static lifetime
    pub fn clone_static(&self) -> NMRemoteConnection<'static> {
        NMRemoteConnection {
            connection: self.connection.clone(),
            owner: None,
        }
    }
}
