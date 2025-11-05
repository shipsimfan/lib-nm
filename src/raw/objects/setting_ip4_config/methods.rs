use std::ffi::CStr;

/// IPv4 configuration should be automatically determined via a method appropriate for the hardware
/// interface, ie DHCP or PPP or some other device-specific manner.
pub const NM_SETTING_IP4_CONFIG_METHOD_AUTO: &CStr = c"auto";

/// IPv4 configuration should be automatically configured for link-local-only operation.
pub const NM_SETTING_IP4_CONFIG_METHOD_LINK_LOCAL: &CStr = c"link-local";

/// All necessary IPv4 configuration (addresses, prefix, DNS, etc) is specified in the setting's
/// properties.
pub const NM_SETTING_IP4_CONFIG_METHOD_MANUAL: &CStr = c"manual";

/// This connection specifies configuration that allows other computers to connect through it to
/// the default network (usually the Internet). The connection's interface will be assigned a
/// private address, and a DHCP server, caching DNS server, and Network Address Translation (NAT)
/// functionality will be started on this connection's interface to allow other devices to connect
/// through that interface to the default network.
pub const NM_SETTING_IP4_CONFIG_METHOD_SHARED: &CStr = c"shared";

/// This connection does not use or require IPv4 address and it should be disabled.
pub const NM_SETTING_IP4_CONFIG_METHOD_DISABLED: &CStr = c"disabled";
