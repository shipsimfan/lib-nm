use crate::{NMSetting, NMSettingWireless};
use std::ops::Deref;

impl<'connection> Deref for NMSettingWireless<'connection> {
    type Target = NMSetting<'connection>;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl<'connection> AsRef<NMSetting<'connection>> for NMSettingWireless<'connection> {
    fn as_ref(&self) -> &NMSetting<'connection> {
        &self.settings
    }
}
