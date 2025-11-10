use glib_2::raw::glib::guint;

/// the state of the connection is unknown
pub const NM_ACTIVE_CONNECTION_STATE_UNKNOWN: guint = 0;

/// a network connection is being prepared
pub const NM_ACTIVE_CONNECTION_STATE_ACTIVATING: guint = 1;

/// there is a connection to the network
pub const NM_ACTIVE_CONNECTION_STATE_ACTIVATED: guint = 2;

/// the network connection is being torn down and cleaned up
pub const NM_ACTIVE_CONNECTION_STATE_DEACTIVATING: guint = 3;

/// the network connection is disconnected and will be removed
pub const NM_ACTIVE_CONNECTION_STATE_DEACTIVATED: guint = 4;
