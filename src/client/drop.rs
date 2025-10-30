use crate::NMClient;
use glib_2::raw::gobject::g_object_unref;

impl Drop for NMClient {
    fn drop(&mut self) {
        unsafe { g_object_unref(self.handle) };
    }
}
