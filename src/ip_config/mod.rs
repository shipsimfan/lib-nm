use glib_2::gobject::GObject;

mod addresses;
mod deref;
mod get;
mod new;

/// IP configuration associated with a device or connection
pub struct NMIPConfig<'owner, Owner> {
    /// The handle to the underlying object
    object: GObject,

    /// The owner of this IP configuration
    owner: Option<&'owner Owner>,
}
