use crate::{NMClient, raw};

mod iter;

mod get;
mod iface;
mod new;
mod r#type;

pub use iter::NMDeviceIter;

/// A device
#[derive(Clone)]
pub struct NMDevice<'client> {
    /// The handle to underlying device
    handle: *mut raw::NMDevice,

    /// The client this device came from
    #[allow(unused)]
    client: &'client NMClient,
}
