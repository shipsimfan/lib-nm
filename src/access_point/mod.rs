use crate::{NMDeviceWifi, raw};

mod iter;

mod bssid;
mod filter_connections;
mod get;
mod max_bitrate;
mod new;
mod rsn_flags;
mod ssid;
mod strength;
mod wpa_flags;

pub use iter::NMAccessPointIter;

/// A Wi-Fi access point
#[derive(Clone)]
pub struct NMAccessPoint<'client, 'device> {
    /// The handle to underlying access point
    handle: *mut raw::NMAccessPoint,

    /// The device this access point came from
    device: &'device NMDeviceWifi<'client>,
}
