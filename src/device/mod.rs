use crate::NMClient;
use glib_2::gobject::GObject;

mod iter;

mod active_connection;
mod clone_static;
mod deref;
mod filter_connections;
mod get;
mod iface;
mod new;
mod r#type;

pub use iter::NMDeviceIter;

/// A device
#[derive(Clone)]
pub struct NMDevice<'client> {
    /// The handle to underlying object
    object: GObject,

    /// The client this device came from
    client: Option<&'client NMClient>,
}
