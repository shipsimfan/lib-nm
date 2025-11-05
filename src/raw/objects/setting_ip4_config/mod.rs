use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMSetting, NMSettingIPConfig};

mod get_type;
mod methods;
mod new;

pub use get_type::nm_setting_ip4_config_get_type;
pub use methods::*;
pub use new::nm_setting_ip4_config_new;

/// Describes IPv4 addressing, routing, and name service properties
///
/// The [`NMSettingIP4Config`] object is a [`NMSetting`] subclass that describes properties related
/// to IPv4 addressing, routing, and Domain Name Service.
///
/// [`NMSettingIP4Config`] has few properties or methods of its own; it inherits almost everything
/// from [`NMSettingIPConfig`].
///
/// NetworkManager supports 5 values for the “method” property for IPv4. If "auto" is specified
/// then the appropriate automatic method (DHCP, PPP, etc) is used for the interface and most other
/// properties can be left unset. If "link-local" is specified, then a link-local address in the
/// 169.254/16 range will be assigned to the interface. If "manual" is specified, static IP
/// addressing is used and at least one IP address must be given in the "addresses" property. If
/// "shared" is specified (indicating that this connection will provide network access to other
/// computers) then the interface is assigned an address in the 10.42.x.1/24 range and a DHCP and
/// forwarding DNS server are started, and the interface is NAT-ed to the current default network
/// connection. "disabled" means IPv4 will not be used on this connection.
pub type NMSettingIP4Config = c_void;
