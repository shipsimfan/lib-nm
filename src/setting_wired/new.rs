use crate::{NMConnection, NMSetting, NMSettingWired, raw};

impl<'connection> NMSettingWired<'connection> {
    /// Create a new [`NMSettingWired`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMSettingWired,
        connection: Option<&'connection NMConnection>,
    ) -> Self {
        let settings = unsafe { NMSetting::new_raw(handle, connection) };
        NMSettingWired { settings }
    }
}
