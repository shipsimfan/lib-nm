use crate::NMClient;
use glib_2::gobject::GObject;

mod deref;
mod get;
mod id;
mod new;

#[allow(missing_docs)]
pub struct NMActiveConnection<'owner, Owner = NMClient> {
    /// The handle to the underlying object
    object: GObject,

    /// The owner of this connection
    owner: Option<&'owner Owner>,
}
