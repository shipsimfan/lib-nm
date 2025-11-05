use crate::NMConnection;

mod deref;
mod new;

/// An unmanaged connection
pub struct NMSimpleConnection {
    /// The underlying connection
    connection: NMConnection,
}
