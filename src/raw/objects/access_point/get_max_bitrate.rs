use crate::raw::NMAccessPoint;
use glib_2::glib::guint32;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the maximum bit rate of the access point in kbit/s.
    ///
    /// # Parameters
    ///  * `ap` - a [`NMAccessPoint`]
    ///
    /// # Returns
    ///  the maximum bit rate (kbit/s)
    pub fn nm_access_point_get_max_bitrate(ap: *mut NMAccessPoint) -> *mut guint32;
}
