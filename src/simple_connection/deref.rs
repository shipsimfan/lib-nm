use crate::{NMConnection, NMSimpleConnection};
use std::ops::Deref;

impl Deref for NMSimpleConnection {
    type Target = NMConnection;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl AsRef<NMConnection> for NMSimpleConnection {
    fn as_ref(&self) -> &NMConnection {
        &self.connection
    }
}
