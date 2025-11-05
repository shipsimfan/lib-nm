use crate::NMSetting;

mod deref;
mod get_gtype;
mod id;
mod new;
mod r#type;
mod uuid;

/// Describes general connection properties
pub struct NMSettingConnection<'connection> {
    /// The underlying setting
    settings: NMSetting<'connection>,
}
