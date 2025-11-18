use crate::{NMClient, NMConnection};

mod iter;

mod clone_static;
mod commit_changes;
mod delete;
mod deref;
mod get;
mod new;

pub use iter::NMRemoteConnectionIter;

/// A connection managed by NetworkManager server
#[derive(Clone)]
pub struct NMRemoteConnection<'owner, Owner = NMClient> {
    /// The underlying connection
    connection: NMConnection,

    /// The owner of this connection
    owner: Option<&'owner Owner>,
}
