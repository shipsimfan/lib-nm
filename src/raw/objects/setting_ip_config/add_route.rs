use crate::{NMIPRoute, NMSettingIPConfig};
use glib_2::glib::gboolean;

// rustdoc imports
#[allow(unused_imports)]
use glib_2::glib::{FALSE, TRUE};

#[link(name = "nm")]
unsafe extern "C" {
    /// Appends a new route and associated information to the setting. The given route is
    /// duplicated internally and is not changed by this function. If an identical route
    /// (considering attributes as well) already exists, the route is not added and the function
    /// returns [`FALSE`].
    ///
    /// Note that before 1.10, this function would not consider route attributes and not add a
    /// route that has an existing route with same `dest`/`prefix`,`next_hop`,`metric` parameters.
    ///
    /// # Parameters
    ///  * `setting` - the [`NMSettingIPConfig`]
    ///  * `route` - the route to add
    ///
    /// # Returns
    /// [`TRUE`] if the route was added; [`FALSE`] if the route was already known.
    pub fn nm_setting_ip_config_add_route(
        setting: *mut NMSettingIPConfig,
        route: *mut NMIPRoute,
    ) -> gboolean;
}
