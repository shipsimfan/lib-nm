/// [`NMDeviceType`] values indicate the type of hardware represented by a device object.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum NMDeviceType {
    /// unknown device
    Unknown = 0,

    /// a wired ethernet device
    Ethernet = 1,

    /// an 802.11 Wi-Fi device
    Wifi = 2,

    /// not used
    Unused1 = 3,

    /// not used
    Unused2 = 4,

    /// a Bluetooth device supporting PAN or DUN access protocols
    Bt = 5,

    /// an OLPC XO mesh networking device
    OlpcMesh = 6,

    /// an 802.16e Mobile WiMAX broadband device
    Wimax = 7,

    /// a modem supporting analog telephone, CDMA/EVDO, GSM/UMTS, or LTE network access protocols
    Modem = 8,

    /// an IP-over-InfiniBand device
    InfiniBand = 9,

    /// a bond controller interface
    Bond = 10,

    /// an 802.1Q VLAN interface
    VLan = 11,

    /// ADSL modem
    Adsl = 12,

    /// a bridge controller interface
    Bridge = 13,

    /// generic support for unrecognized device types
    Generic = 14,

    /// a team controller interface
    Team = 15,

    /// a TUN or TAP interface
    Tun = 16,

    /// a IP tunnel interface
    IpTunnel = 17,

    /// a MACVLAN interface
    MacVLan = 18,

    /// a VXLAN interface
    VxLan = 19,

    /// a VETH interface
    VEth = 20,

    /// a MACsec interface
    MacSec = 21,

    /// a dummy interface
    Dummy = 22,

    /// a PPP interface
    Ppp = 23,

    /// a Open vSwitch interface
    OvsInterface = 24,

    /// a Open vSwitch port
    OvsPort = 25,

    /// a Open vSwitch bridge
    OvsBridge = 26,

    /// a IEEE 802.15.4 (WPAN) MAC Layer Device
    WPan = 27,

    /// 6LoWPAN interface
    _6LoWPan = 28,

    /// a WireGuard interface
    Wireguard = 29,

    /// an 802.11 Wi-Fi P2P device.
    WifiP2p = 30,

    /// A VRF (Virtual Routing and Forwarding) interface.
    Vrf = 31,

    /// a loopback interface.
    Loopback = 32,
}
