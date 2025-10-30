use crate::NMDevice;

mod access_points;
mod deref;
mod new;

/// A Wi-Fi device
#[derive(Clone)]
pub struct NMDeviceWifi<'client> {
    /// The underlying device
    device: NMDevice<'client>,
}
