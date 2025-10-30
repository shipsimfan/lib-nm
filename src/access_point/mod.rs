use crate::{NMDeviceWifi, raw};

mod iter;

mod bssid;
mod max_bitrate;
mod new;
mod ssid;
mod strength;

pub use iter::NMAccessPointIter;

/// A Wi-Fi access point
#[derive(Clone)]
pub struct NMAccessPoint<'client, 'device> {
    /// The handle to underlying access point
    handle: *mut raw::NMAccessPoint,

    /// The device this access point came from
    #[allow(unused)]
    device: &'device NMDeviceWifi<'client>,
}
