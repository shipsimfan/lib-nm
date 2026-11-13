use crate::{NMSetting, NMSettingWired};
use std::ops::Deref;

impl<'connection> Deref for NMSettingWired<'connection> {
    type Target = NMSetting<'connection>;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl<'connection> AsRef<NMSetting<'connection>> for NMSettingWired<'connection> {
    fn as_ref(&self) -> &NMSetting<'connection> {
        &self.settings
    }
}
