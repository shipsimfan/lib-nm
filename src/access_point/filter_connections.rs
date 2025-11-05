use crate::{NMAccessPoint, NMRemoteConnectionIter, raw::nm_access_point_filter_connections};

impl<'client, 'device> NMAccessPoint<'client, 'device> {
    /// Filters a given array of connections for a given [`NMAccessPoint`] object and returns
    /// connections which may be activated with the access point. Any returned connections will
    /// match the access points's SSID and (if given) BSSID and other attributes like security
    /// settings, channel, etc.
    pub fn filter_connections<'client2, Owner>(
        &self,
        iter: &NMRemoteConnectionIter<'client2, Owner>,
    ) -> NMRemoteConnectionIter<'client2, Owner> {
        let handle = unsafe { nm_access_point_filter_connections(self.handle, iter.handle()) };
        unsafe { NMRemoteConnectionIter::new(handle, iter.owner(), true) }
    }
}
