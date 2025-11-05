use crate::NMClient;
use glib_2::gobject::GObject;
use std::ops::Deref;

impl Deref for NMClient {
    type Target = GObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl AsRef<GObject> for NMClient {
    fn as_ref(&self) -> &GObject {
        &self.object
    }
}
