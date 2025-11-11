use crate::{NMIPConfig, raw};
use glib_2::{gobject::GObject, util::NewRaw};

impl<'owner, Owner> NMIPConfig<'owner, Owner> {
    /// Create a new [`NMIPConfig`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMIPConfig,
        owner: Option<&'owner Owner>,
        owned: bool,
    ) -> Self {
        let object = unsafe { GObject::new_raw(handle, owned) };
        NMIPConfig { object, owner }
    }
}

impl<'owner, Owner> NewRaw for NMIPConfig<'owner, Owner> {
    unsafe fn new_raw(handle: *mut glib_2::raw::gobject::GObject, owned: bool) -> Self {
        unsafe { NMIPConfig::new_raw(handle, None, owned) }
    }
}
