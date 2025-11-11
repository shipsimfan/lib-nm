use crate::raw;

mod iter;

mod address;
mod drop;
mod get;
mod new;
mod prefix;

pub use iter::NMIPAddressIter;

/// An IP address
pub struct NMIPAddress<'owner, Owner> {
    /// The handle to underlying access point
    handle: *mut raw::NMAccessPoint,

    /// The setting this IP address came from
    setting: Option<&'owner Owner>,

    /// Is this IP address owned?
    owned: bool,
}
