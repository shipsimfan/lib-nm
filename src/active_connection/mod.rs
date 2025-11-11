use crate::NMClient;
use glib_2::gobject::GObject;

mod clone_static;
mod connection;
mod deref;
mod get;
mod id;
mod ip4_config;
mod new;
mod on_state_changed;

pub use on_state_changed::NMActiveConnectionStateChangedCallback;

#[allow(missing_docs)]
#[derive(Clone)]
pub struct NMActiveConnection<'owner, Owner = NMClient> {
    /// The handle to the underlying object
    object: GObject,

    /// The owner of this connection
    owner: Option<&'owner Owner>,
}
