use glib_2::raw::gobject::GType;

#[link(name = "nm")]
unsafe extern "C" {
    #[allow(missing_docs)]
    pub fn nm_setting_wireless_get_type() -> GType;
}
