use crate::{NMDevice, NMRemoteConnectionIter, raw::nm_device_filter_connections};

// rustdoc imports
#[allow(unused_imports)]
use crate::NMClient;

impl<'client> NMDevice<'client> {
    /// Filters a given array of connections for a given [`NMDevice`] object and returns
    /// connections which may be activated with the device. For example if device is a Wi-Fi device
    /// that supports only WEP encryption, the returned array will contain any Wi-Fi connections in
    /// connections that allow connection to unencrypted or WEP-enabled SSIDs. The returned array
    /// will not contain Ethernet, Bluetooth, Wi-Fi WPA connections, or any other connection that
    /// is incompatible with the device. To get the full list of connections see
    /// [`NMClient::connections`].
    pub fn filter_connections<'client2, Owner>(
        &self,
        iter: &NMRemoteConnectionIter<'client2, Owner>,
    ) -> NMRemoteConnectionIter<'client2, Owner> {
        let handle = unsafe { nm_device_filter_connections(self.handle, iter.handle()) };
        unsafe { NMRemoteConnectionIter::new(handle, iter.owner(), true) }
    }
}
