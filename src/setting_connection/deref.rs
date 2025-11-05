use crate::{NMSetting, NMSettingConnection};
use std::ops::Deref;

impl<'connection> Deref for NMSettingConnection<'connection> {
    type Target = NMSetting<'connection>;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl<'connection> AsRef<NMSetting<'connection>> for NMSettingConnection<'connection> {
    fn as_ref(&self) -> &NMSetting<'connection> {
        &self.settings
    }
}
