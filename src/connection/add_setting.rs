use crate::{NMConnection, NMSetting, raw::nm_connection_add_setting};

impl NMConnection {
    /// Adds a [`NMSetting`] to the connection, replacing any previous [`NMSetting`] of the same
    /// name which has previously been added to the [`NMConnection`]
    pub fn add_setting<'a, T: AsRef<NMSetting<'a>>>(&self, setting: T) {
        let setting_ref = setting.as_ref();
        assert!(setting_ref.connection().is_none());
        unsafe { nm_connection_add_setting(self.handle(), setting_ref.handle()) };

        let _ = std::mem::ManuallyDrop::new(setting);
    }
}
