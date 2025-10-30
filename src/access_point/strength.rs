use crate::{NMAccessPoint, raw::nm_access_point_get_strength};

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Get the strength of this access point, as a percentage from 0 - 100
    pub fn strength(&self) -> u8 {
        unsafe { nm_access_point_get_strength(self.handle) }
    }
}
