use crate::{NMClient, NMConnection};

mod iter;

mod deref;
mod get;
mod new;

pub use iter::NMRemoteConnectionIter;

/// A connection managed by NetworkManager server
pub struct NMRemoteConnection<'owner, Owner = NMClient> {
    /// The underlying connection
    connection: NMConnection,

    /// The owner of this connection
    owner: Option<&'owner Owner>,
}
