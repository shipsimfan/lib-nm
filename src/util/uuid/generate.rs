use crate::{raw::nm_utils_uuid_generate, util::UUID};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::NMSettingConnection;

impl UUID {
    /// Generate a newly allocated UUID suitable for use as the [`NMSettingConnection`] object's
    /// “id”: property.
    pub fn generate() -> UUID {
        let ptr = unsafe { nm_utils_uuid_generate() };
        assert_ne!(ptr, null_mut());
        UUID { ptr }
    }
}
