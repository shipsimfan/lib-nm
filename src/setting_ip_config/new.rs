use crate::{NMConnection, NMSetting, NMSettingIPConfig, raw};

impl<'connection> NMSettingIPConfig<'connection> {
    /// Create a new [`NMSettingIPConfig`] from a raw `handle`
    pub unsafe fn new_raw(
        handle: *mut raw::NMSettingIPConfig,
        connection: Option<&'connection NMConnection>,
    ) -> Self {
        let settings = unsafe { NMSetting::new_raw(handle, connection) };
        NMSettingIPConfig { settings }
    }
}
