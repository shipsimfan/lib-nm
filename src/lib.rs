//! libnm (Network Manager) bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod raw;

pub use raw::NMDeviceType;

mod access_point;
mod client;
mod device;
mod device_wifi;

pub use access_point::{NMAccessPoint, NMAccessPointIter};
pub use client::NMClient;
pub use device::{NMDevice, NMDeviceIter};
pub use device_wifi::NMDeviceWifi;
