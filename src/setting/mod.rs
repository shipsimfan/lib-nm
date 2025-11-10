use crate::NMConnection;
use glib_2::gobject::GObject;

mod deref;
mod get;
mod new;

/// Describes related configuration information
pub struct NMSetting<'connection> {
    /// The handle to underlying object
    object: GObject,

    /// The connection this setting is for
    connection: Option<&'connection NMConnection>,
}
