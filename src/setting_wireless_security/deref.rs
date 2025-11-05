use crate::{NMSetting, NMSettingWirelessSecurity};
use std::ops::Deref;

impl<'connection> Deref for NMSettingWirelessSecurity<'connection> {
    type Target = NMSetting<'connection>;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl<'connection> AsRef<NMSetting<'connection>> for NMSettingWirelessSecurity<'connection> {
    fn as_ref(&self) -> &NMSetting<'connection> {
        &self.settings
    }
}
