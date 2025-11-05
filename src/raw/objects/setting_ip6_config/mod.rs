use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMSetting, NMSettingIPConfig};

mod get_type;
mod methods;
mod new;

pub use get_type::nm_setting_ip6_config_get_type;
pub use methods::*;
pub use new::nm_setting_ip6_config_new;

/// Describes IPv6 addressing, routing, and name service properties
///
/// The [`NMSettingIP6Config`] object is a [`NMSetting`] subclass that describes properties related
/// to IPv6 addressing, routing, and Domain Name Service
///
/// [`NMSettingIP6Config`] has few properties or methods of its own; it inherits almost everything
/// from [`NMSettingIPConfig`].
///
/// NetworkManager supports 7 values for the “method” property for IPv6. If "auto" is specified
/// then the appropriate automatic method (PPP, router advertisement, etc) is used for the device
/// and most other properties can be left unset. To force the use of DHCP only, specify "dhcp";
/// this method is only valid for Ethernet- based hardware. If "link-local" is specified, then an
/// IPv6 link-local address will be assigned to the interface. If "manual" is specified, static IP
/// addressing is used and at least one IP address must be given in the "addresses" property. If
/// "ignore" is specified, IPv6 configuration is not done. Note: the "shared" method is not yet
/// supported. If "disabled" is specified, IPv6 is disabled completely for the interface.
pub type NMSettingIP6Config = c_void;
