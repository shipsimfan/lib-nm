use crate::{NMSetting, NMSettingIPConfig};
use std::ops::Deref;

impl<'connection> Deref for NMSettingIPConfig<'connection> {
    type Target = NMSetting<'connection>;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl<'connection> AsRef<NMSetting<'connection>> for NMSettingIPConfig<'connection> {
    fn as_ref(&self) -> &NMSetting<'connection> {
        &self.settings
    }
}
