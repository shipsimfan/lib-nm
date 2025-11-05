use crate::{NMConnection, NMRemoteConnection};
use std::ops::Deref;

impl<'owner, Owner> Deref for NMRemoteConnection<'owner, Owner> {
    type Target = NMConnection;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl<'owner, Owner> AsRef<NMConnection> for NMRemoteConnection<'owner, Owner> {
    fn as_ref(&self) -> &NMConnection {
        &self.connection
    }
}
