use crate::NMConnection;
use glib_2::gobject::GObject;
use std::ops::Deref;

impl Deref for NMConnection {
    type Target = GObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl AsRef<GObject> for NMConnection {
    fn as_ref(&self) -> &GObject {
        &self.object
    }
}
