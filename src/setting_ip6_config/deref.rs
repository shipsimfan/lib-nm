use crate::{NMSetting, NMSettingIP6Config, NMSettingIPConfig};
use std::ops::Deref;

impl<'connection> Deref for NMSettingIP6Config<'connection> {
    type Target = NMSettingIPConfig<'connection>;

    fn deref(&self) -> &Self::Target {
        &self.ip_config_settings
    }
}

impl<'connection> AsRef<NMSettingIPConfig<'connection>> for NMSettingIP6Config<'connection> {
    fn as_ref(&self) -> &NMSettingIPConfig<'connection> {
        &self.ip_config_settings
    }
}

impl<'connection> AsRef<NMSetting<'connection>> for NMSettingIP6Config<'connection> {
    fn as_ref(&self) -> &NMSetting<'connection> {
        &self.ip_config_settings
    }
}
