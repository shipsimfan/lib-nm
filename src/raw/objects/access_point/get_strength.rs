use crate::raw::NMAccessPoint;
use glib_2::glib::guint8;

#[link(name = "nm")]
unsafe extern "C" {
    /// Gets the current signal strength of the access point as a percentage.
    ///
    /// # Parameters
    ///  * `ap` - a [`NMAccessPoint`]
    ///
    /// # Returns
    ///  the signal strength (0 to 100)
    pub fn nm_access_point_get_strength(ap: *mut NMAccessPoint) -> guint8;
}
