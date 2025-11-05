use crate::util::UUID;
use glib_2::raw::glib::g_free;

impl Drop for UUID {
    fn drop(&mut self) {
        unsafe { g_free(self.ptr.cast()) };
    }
}
