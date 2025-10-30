use crate::{NMAccessPoint, raw::nm_access_point_get_max_bitrate};

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Get the max bitrate of this access point, in KBit/s
    pub fn max_bitrate(&self) -> u32 {
        unsafe { nm_access_point_get_max_bitrate(self.handle) }
    }
}
