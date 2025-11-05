use crate::{NMSetting, NMSettingIP4Config, NMSettingIPConfig};
use std::ops::Deref;

impl<'connection> Deref for NMSettingIP4Config<'connection> {
    type Target = NMSettingIPConfig<'connection>;

    fn deref(&self) -> &Self::Target {
        &self.ip_config_settings
    }
}

impl<'connection> AsRef<NMSettingIPConfig<'connection>> for NMSettingIP4Config<'connection> {
    fn as_ref(&self) -> &NMSettingIPConfig<'connection> {
        &self.ip_config_settings
    }
}

impl<'connection> AsRef<NMSetting<'connection>> for NMSettingIP4Config<'connection> {
    fn as_ref(&self) -> &NMSetting<'connection> {
        &self.ip_config_settings
    }
}
