use glib_2::gobject::GType;

/// An object which has an associated [`GType`]
pub trait GetGType {
    /// Get the associated [`GType`]
    fn get_gtype() -> GType;
}
