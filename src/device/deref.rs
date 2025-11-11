use crate::NMDevice;
use glib_2::gobject::GObject;
use std::ops::Deref;

impl<'client> Deref for NMDevice<'client> {
    type Target = GObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<'client> AsRef<GObject> for NMDevice<'client> {
    fn as_ref(&self) -> &GObject {
        &self.object
    }
}
