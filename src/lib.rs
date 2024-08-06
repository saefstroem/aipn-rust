use num_enum::FromPrimitive;

/**
### This enum contains all the different types of protocol numbers that can be found in the protocol field of an IPv4 packet.

If you find a mistake or want to add a new variant, please open an issue or a pull request.

Reference: https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml

#### Example
```rust
use aipn::AIPN;

let protocol = AIPN::TCP;
println!("Protocol type: {:?}", protocol);

match protocol {
    AIPN::TCP => {
        // Adjust code execution according to the protocol
    },
    _ => println!("Unknown protocol"),
}

let some_protocol_value=17;
let protocol=AIPN::from_u8(some_protocol_value);
println!("Protocol type: {:?}", protocol);
```

 */
#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum AIPN {
    /// IPv6 Hop-by-Hop Option
    HOPOPT = 0, 
    /// Internet Control Message Protocol
    ICMP = 1,
    /// Internet Group Management Protocol
    IGMP = 2,
    /// Gateway-to-Gateway
    GGP = 3,
    /// IPv4 encapsulation
    IPv4 = 4,
    /// Stream
    ST = 5,
    /// Transmission Control Protocol
    TCP = 6,
    /// CBT
    CBT = 7,
    /// Exterior Gateway Protocol
    EGP = 8,
    /// any private interior gateway (used by Cisco for their IGRP)
    IGP = 9,
    /// BBN RCC Monitoring
    BbnRccMon = 10,
    /// Network Voice Protocol
    NvpIi = 11,
    /// PUP
    PUP = 12,
    /// ARGUS
    ARGUS = 13,
    /// EMCON
    EMCON = 14,
    /// Cross Net Debugger
    XNET = 15,
    /// Chaos
    CHAOS = 16,
    /// User Datagram Protocol
    UDP = 17,
    /// Multiplexing
    MUX = 18,
    /// DCN Measurement Subsystems
    DcnMeas = 19,
    /// Host Monitoring
    HMP = 20,
    /// Packet Radio Measurement
    PRM = 21,
    /// XEROS NS IDP
    XnsIdp = 22,
    /// Trunk-1
    Trunk1 = 23,
    /// Trunk-2
    Trunk2 = 24,
    /// Leaf-1
    Leaf1 = 25,
    /// Leaf-2
    Leaf2 = 26,
    /// Reliable Data Protocol
    RDP = 27,
    /// Internet Reliable Transaction
    IRTP = 28,
    /// ISO Transport Protocol Class 4
    IsoTp4 = 29,
    /// Bulk Data Transfer Protocol
    NETBLT = 30,
    /// MFE Network Services Protocol
    MfeNsp = 31,
    /// Merit Internodal Protocol
    MeritInp = 32,
    /// Datagram Congestion Control Protocol
    DCCP = 33,
    /// Third Party Connect Protocol
    ThreePC = 34,
    /// Inter-Domain Policy Routing Protocol
    IDPR = 35,
    /// XTP
    XTP = 36,
    /// Datagram Delivery Protocol
    DDP = 37,
    /// IDPR Control Message Transport Protocol
    IdprCmtp = 38,
    /// TP++ Transport Protocol
    TpPlusPlus = 39,
    /// IL Transport Protocol
    IL = 40,
    /// IPv6 Encapsulation
    IPv6 = 41,
    /// Source Demand Routing Protocol
    SDRP = 42,
    /// Routing Header for IPv6
    Ipv6Route = 43,
    /// Fragment Header for IPv6
    Ipv6Frag = 44,
    /// Inter-Domain Routing Protocol
    IDRP = 45,
    /// Reservation Protocol
    RSVP = 46,
    /// General Routing Encapsulation
    GRE = 47,
    /// Dynamic Source Routing Protocol
    DSR = 48,
    /// BNA
    BNA = 49,
    /// Encap Security Payload
    ESP = 50,
    /// Authentication Header
    AH = 51,
    /// Integrated Net Layer Security TUBA
    INlsp = 52,
    /// IP with Encryption
    SWIPE = 53,
    /// NBMA Address Resolution Protocol
    NARP = 54,
    /// Minimal Ipv4 Encapsulation
    MinIpv4 = 55,
    /// Transportation Layer Security Protocol using Kryptonet key management
    TLSP = 56,
    /// SKIP
    SKIP = 57,
    /// Ipv6 Icmp
    Ipv6Icmp = 58,
    /// Ipv6 No Next Header
    Ipv6NoNxt = 59,
    /// Ipv6 Destination Options
    Ipv6Opts = 60,
    /// Any host internal protocol
    AnyHostInternalProtocol = 61,
    /// CFTP
    CFTP = 62,
    /// Any local network
    AnyLocalNetwork = 63,
    /// SATNET and Backroom EXPAK
    SatExpak = 64,
    /// Kryptolan
    KRYPTOLAN = 65,
    /// MIT Remote Virtual Disk Protocol
    RDVI = 66,
    /// Internet Pluribus Packet Core
    IPPC = 67,
    /// Any distributed file system
    AnyDistributedFileSystem = 68,
    /// SATNET Monitoring
    SatMon = 69,
    /// VISA Protocol
    VISA = 70,
    /// Internet Packet Core Utility
    IPCV = 71,
    /// Computer Protocol Network Executive
    CPNX = 72,
    /// Computer Protocol Heart Beat
    CPHB = 73,
    /// Wang Span Network
    WSN = 74,
    /// Packet Video Protocol
    PVP = 75,
    /// Backroom SATNET Monitoring
    BrSatMon = 76,
    /// SUN ND PROTOCOL-Temporary
    SunNd = 77,
    /// WIDEBAND Monitoring
    WbMon = 78,
    /// WIDEBAND EXPAK
    WbExpak = 79,
    /// ISO Internet Protocol
    IsoIp = 80,
    /// VMTP
    VMTP = 81,
    /// SECURE-VMTP
    SecureVmtp = 82,
    /// VINES
    VINES = 83,
    /// Internet Protocol Traffic Manager
    IPTM = 84,
    /// NSFNET-IGP
    NsfnetIgp = 85,
    /// Dissimilar Gateway Protocol
    DGP = 86,
    /// TCF
    TCF = 87,
    /// EIGRP
    EIGRP = 88,
    /// OSPFIGP
    OSPFIGP = 89,
    /// Sprite RPC Protocol
    SpriteRpc = 90,
    /// Locus Address Resolution Protocol
    LARP = 91,
    /// Multicast Transport Protocol
    MTP = 92,
    /// AX.25 Frames
    AX25 = 93,
    /// IP-within-IP Encapsulation Protocol
    IPIP = 94,
    /// Mobile Internetworking Control Protocol
    MICP = 95,
    /// Semaphore Communications Sec. Pro
    SccSp = 96,
    /// Ethernet-within-IP Encapsulation
    ETHERIP = 97,
    /// Encapsulation Header
    ENCAP = 98,
    /// Any private encryption scheme
    AnyPrivateEncryptionScheme = 99,
    /// GMTP
    GMTP = 100,
    /// Ipsilon Flow Management Protocol
    IFMP = 101,
    /// PNNI over IP
    PNNI = 102,
    /// Protocol Independent Multicast
    PIM = 103,
    /// ARIS
    ARIS = 104,
    /// SCPS
    SCP = 105,
    /// QNX
    QNX = 106,
    /// Active Networks
    AN = 107,
    /// IP Payload Compression Protocol
    IPComp = 108,
    /// Sitara Networks Protocol
    SNP = 109,
    /// Compaq Peer Protocol
    CompaqPeer = 110,
    /// IPX in IP
    IpxInIp = 111,
    /// Virtual Router Redundancy Protocol
    VRRP = 112,
    /// PGM Reliable Transport Protocol
    PGM = 113,
    /// Any 0-hop protocol
    Any0HopProtocol = 114,
    /// Layer Two Tunneling Protocol
    L2TP = 115,
    /// D-II Data Exchange (DDX)
    DDX = 116,
    /// Interactive Agent Transfer Protocol
    IATP = 117,
    /// Schedule Transfer Protocol
    STP = 118,
    /// SpectraLink Radio Protocol
    SRP = 119,
    /// UTI
    UTI = 120,
    /// Simple Message Protocol
    SMP = 121,
    /// Simple Multicast Protocol
    SM = 122,
    /// Performance Transparency Protocol
    PTP = 123,
    /// ISIS over IPv4
    IsisOverIpv4 = 124,
    /// FIRE
    FIRE = 125,
    /// Combat Radio Transport Protocol
    CRTP = 126,
    /// Combat Radio User Datagram
    CRUDP = 127,
    /// SSCOPMCE
    SSCOPMCE = 128,
    /// IPLT
    IPLT = 129,
    /// Secure Packet Shield
    SPS = 130,
    /// Private IP Encapsulation within IP
    PIPE = 131,
    /// Stream Control Transmission Protocol
    SCTP = 132,
    /// Fibre Channel
    FC = 133,
    /// RSVP-E2E-IGNORE
    RsvpE2eIgnore = 134,
    /// Mobility Header
    MobilityHeader = 135,
    /// UDPLite
    UDPLite = 136,
    /// MPLS-in-IP
    MplsInIp = 137,
    /// MANET Protocols
    MANETProtocols = 138,
    /// Host Identity Protocol
    HIP = 139,
    /// Shim6 Protocol
    Shim6 = 140,
    /// Wrapped Encapsulating Security Payload
    WESP = 141,
    /// Robust Header Compression
    ROHC = 142,
    /// Ethernet
    Ethernet = 143,
    /// AGGFRAG Encapsulation payload for ESP
    AGGFRAG = 144,
    /// Network Service Header 
    NSH = 145,
    /// Unknown
    #[num_enum(default)]
    Unknown = u8::MAX,
}

impl AIPN {
    pub fn from_u8(number: u8) -> AIPN {
        Self::from_primitive(number)
    }
}
