use crate::raw::NMConnection;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{NMSetting, NMSimpleConnection};

#[link(name = "nm")]
unsafe extern "C" {
    /// Creates a new [`NMSimpleConnection`] object with no [`NMSetting`] objects.
    ///
    /// # Returns
    /// the new empty [`NMConnection`] object.
    pub fn nm_simple_connection_new() -> *mut NMConnection;
}
