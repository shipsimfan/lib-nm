use std::ffi::CStr;

/// IPv6 is not required or is handled by some other mechanism, and NetworkManager should not
/// configure IPv6 for this connection.
pub const NM_SETTING_IP6_CONFIG_METHOD_IGNORE: &CStr = c"ignore";

/// IPv6 configuration should be automatically determined via a method appropriate for the hardware
/// interface, ie router advertisements, DHCP, or PPP or some other device-specific manner.
pub const NM_SETTING_IP6_CONFIG_METHOD_AUTO: &CStr = c"auto";

/// IPv6 configuration should be automatically determined via DHCPv6 only and router advertisements
/// should be ignored.
pub const NM_SETTING_IP6_CONFIG_METHOD_DHCP: &CStr = c"dhcp";

/// IPv6 configuration should be automatically configured for link-local-only operation.
pub const NM_SETTING_IP6_CONFIG_METHOD_LINK_LOCAL: &CStr = c"link-local";

/// All necessary IPv6 configuration (addresses, prefix, DNS, etc) is specified in the setting's
/// properties.
pub const NM_SETTING_IP6_CONFIG_METHOD_MANUAL: &CStr = c"manual";

/// This connection specifies configuration that allows other computers to connect through it to
/// the default network (usually the Internet).  The connection's interface will be assigned a
/// private address, and router advertisements, a caching DNS server, and Network Address
/// Translation (NAT) functionality will be started on this connection's interface to allow other
/// devices to connect through that interface to the default network. (not yet supported for IPv6)
pub const NM_SETTING_IP6_CONFIG_METHOD_SHARED: &CStr = c"shared";

/// IPv6 is disabled for the connection.
pub const NM_SETTING_IP6_CONFIG_METHOD_DISABLED: &CStr = c"disabled";
