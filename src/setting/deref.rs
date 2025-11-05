use crate::NMSetting;
use glib_2::gobject::GObject;
use std::ops::Deref;

impl<'connection> Deref for NMSetting<'connection> {
    type Target = GObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<'connection> AsRef<GObject> for NMSetting<'connection> {
    fn as_ref(&self) -> &GObject {
        &self.object
    }
}
