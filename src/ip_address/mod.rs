use crate::{NMSetting, raw};

mod address;
mod drop;
mod get;
mod new;
mod prefix;

/// An IP address
pub struct NMIPAddress<'setting, 'connection> {
    /// The handle to underlying access point
    handle: *mut raw::NMAccessPoint,

    /// The setting this IP address came from
    setting: Option<&'setting NMSetting<'connection>>,

    /// Is this IP address owned?
    owned: bool,
}
