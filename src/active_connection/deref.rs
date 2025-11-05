use crate::NMActiveConnection;
use glib_2::gobject::GObject;
use std::ops::Deref;

impl<'owner, Owner> Deref for NMActiveConnection<'owner, Owner> {
    type Target = GObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<'owner, Owner> AsRef<GObject> for NMActiveConnection<'owner, Owner> {
    fn as_ref(&self) -> &GObject {
        &self.object
    }
}
