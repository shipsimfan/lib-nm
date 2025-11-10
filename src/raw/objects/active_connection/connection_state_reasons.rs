use glib_2::raw::glib::guint;

/// The reason for the active connection state change is unknown.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_UNKNOWN: guint = 0;

/// No reason was given for the active connection state change.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_NONE: guint = 1;

/// The active connection changed state because the user disconnected it.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_USER_DISCONNECTED: guint = 2;

/// The active connection changed state because the device it was using was disconnected.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_DEVICE_DISCONNECTED: guint = 3;

/// The service providing the VPN connection was stopped.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_SERVICE_STOPPED: guint = 4;

/// The IP config of the active connection was invalid.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_IP_CONFIG_INVALID: guint = 5;

/// The connection attempt to the VPN service timed out.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_CONNECT_TIMEOUT: guint = 6;

/// A timeout occurred while starting the service providing the VPN connection.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_SERVICE_START_TIMEOUT: guint = 7;

/// Starting the service providing the VPN connection failed.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_SERVICE_START_FAILED: guint = 8;

/// Necessary secrets for the connection were not provided.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_NO_SECRETS: guint = 9;

/// Authentication to the server failed.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_LOGIN_FAILED: guint = 10;

/// The connection was deleted from settings.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_CONNECTION_REMOVED: guint = 11;

/// Master connection of this connection failed to activate.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_DEPENDENCY_FAILED: guint = 12;

/// Could not create the software device link.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_DEVICE_REALIZE_FAILED: guint = 13;

/// The device this connection depended on disappeared.
pub const NM_ACTIVE_CONNECTION_STATE_REASON_DEVICE_REMOVED: guint = 14;
