use std::ffi::CStr;

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_METHOD: &CStr = c"method";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DNS: &CStr = c"dns";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DNS_SEARCH: &CStr = c"dns-search";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DNS_OPTIONS: &CStr = c"dns-options";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DNS_PRIORITY: &CStr = c"dns-priority";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_ADDRESSES: &CStr = c"addresses";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_GATEWAY: &CStr = c"gateway";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_ROUTES: &CStr = c"routes";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_ROUTE_METRIC: &CStr = c"route-metric";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_ROUTE_TABLE: &CStr = c"route-table";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_IGNORE_AUTO_ROUTES: &CStr = c"ignore-auto-routes";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_IGNORE_AUTO_DNS: &CStr = c"ignore-auto-dns";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DHCP_HOSTNAME: &CStr = c"dhcp-hostname";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DHCP_SEND_HOSTNAME: &CStr = c"dhcp-send-hostname";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DHCP_HOSTNAME_FLAGS: &CStr = c"dhcp-hostname-flags";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_NEVER_DEFAULT: &CStr = c"never-default";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_MAY_FAIL: &CStr = c"may-fail";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DAD_TIMEOUT: &CStr = c"dad-timeout";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DHCP_TIMEOUT: &CStr = c"dhcp-timeout";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_REQUIRED_TIMEOUT: &CStr = c"required-timeout";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DHCP_IAID: &CStr = c"dhcp-iaid";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_DHCP_REJECT_SERVERS: &CStr = c"dhcp-reject-servers";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_AUTO_ROUTE_EXT_GW: &CStr = c"auto-route-ext-gw";

#[allow(missing_docs)]
pub const NM_SETTING_IP_CONFIG_REPLACE_LOCAL_RULE: &CStr = c"replace-local-rule";
