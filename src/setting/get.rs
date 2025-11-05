use crate::{NMConnection, NMSetting};

impl<'connection> NMSetting<'connection> {
    /// Get the [`NMConnection`] this setting belongs to
    pub fn connection(&self) -> Option<&'connection NMConnection> {
        self.connection
    }
}
