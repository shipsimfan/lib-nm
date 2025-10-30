use crate::raw;

mod devices;
mod drop;
mod get;
mod new;
mod version;

/// [`NMClient`] contains a cache of the objects of NetworkManager's D-Bus API.
pub struct NMClient {
    /// The handle to underlying client
    handle: *mut raw::NMClient,
}
