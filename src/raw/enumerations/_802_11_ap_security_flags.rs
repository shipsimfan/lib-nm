/// 802.11 access point security and authentication flags. These flags describe the current
/// security requirements of an access point as determined from the access point's beacon.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum NM80211ApSecurityFlags {
    /// the access point has no special security requirements
    None = 0x00000000,

    /// 40/64-bit WEP is supported for pairwise/unicast encryption
    PairWep40 = 0x00000001,

    /// 104/128-bit WEP is supported for pairwise/unicast encryption
    PairWep104 = 0x00000002,

    /// TKIP is supported for pairwise/unicast encryption
    PairTkip = 0x00000004,

    /// AES/CCMP is supported for pairwise/unicast encryption
    PairCcmp = 0x00000008,

    /// 40/64-bit WEP is supported for group/broadcast encryption
    GroupWep40 = 0x00000010,

    /// 104/128-bit WEP is supported for group/broadcast encryption
    GroupWep104 = 0x00000020,

    /// TKIP is supported for group/broadcast encryption
    GroupTkip = 0x00000040,

    /// AES/CCMP is supported for group/broadcast encryption
    GroupCcmp = 0x00000080,

    /// WPA/RSN Pre-Shared Key encryption is supported
    KeyMgmtPsk = 0x00000100,

    /// 802.1x authentication and key management is supported
    KeyMgmt8021x = 0x00000200,

    /// WPA/RSN Simultaneous Authentication of Equals is supported
    KeyMgmtSae = 0x00000400,

    /// WPA/RSN Opportunistic Wireless Encryption is supported
    KeyMgmtOwe = 0x00000800,

    /// WPA/RSN Opportunistic Wireless Encryption transition mode is supported. Since: 1.26.
    KeyMgmtOweTm = 0x00001000,

    /// WPA3 Enterprise Suite-B 192 bit mode is supported. Since: 1.30.
    KeyMgmtEapSuiteB192 = 0x00002000,
}

impl NM80211ApSecurityFlags {
    /// Does this flags instance contain `flag`
    pub fn contains(&self, flag: NM80211ApSecurityFlags) -> bool {
        (*self as u32 & flag as u32) != 0
    }
}
