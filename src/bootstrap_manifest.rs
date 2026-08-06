//! Producer-owned authority state for the Agent meta-policy Interface.
//!
//! Every identity and canonical-order value is an allocated opaque seat.
//! None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    109, 119, 119, 221, 153, 15, 55, 92, 176, 115, 129, 215, 244, 22, 224, 123, 25, 202, 254, 32,
    100, 96, 251, 238, 114, 230, 145, 16, 2, 244, 220, 37,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 4913;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 52230;

pub const INTERFACE_SEAT: AuthoritySeat = AuthoritySeat::new("Interface", 4231, 0x79a12c918f40ae29);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 50697, 0x8744b01d0e31a583);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 20604, 0x39277f61f3f0986d);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 18369, 0x6af07109b399d167);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 19782, 0xa246f6304dc59ef1);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 47939, 0xe723881b939ff38b);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 18494, 0xdbbd2e308e8905b5);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 3499, 0x236104afbd66efef);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 60834, 0x46b3e5b16d1050b9);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 8958, 0xd66eccc19f77ea93);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 43113, 0x28be67fc4d8143fd);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 26139, 0xbce61da761a64777);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 35673, 0x24361bde25d5e381);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 28658, 0xd4a7f2d31533aa9b);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 41814, 0xfc7e9f20a2a07345);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 47603, 0x7b4332699f33f7ff);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    50940, 4111, 36809, 33073, 14192, 52017, 46152, 58384, 42610, 16250,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "AgentMetaRequest", 28066, 0x7d55fa3c893305ab),
    DeclarationSeat::new(None, "AgentMetaReply", 30134, 0xfbda8c9db48424d5),
    DeclarationSeat::new(None, "ProviderName", 60640, 0x134e9d5785dee40f),
    DeclarationSeat::new(None, "EndpointUrl", 17268, 0xd05a2815701721d9),
    DeclarationSeat::new(None, "ModelName", 17552, 0x84daddb3ecdee0b3),
    DeclarationSeat::new(None, "EnvironmentVariable", 6204, 0xcd5ef68cf06ae71d),
    DeclarationSeat::new(None, "GopassPath", 25274, 0xa061c1c8ca685f97),
    DeclarationSeat::new(None, "SecretFilePath", 51727, 0x0ac19cd542ab78a1),
    DeclarationSeat::new(None, "RejectionDetail", 49688, 0x5780a15c163f04bb),
    DeclarationSeat::new(None, "EnvironmentSecret", 26795, 0x61c2b878b9bd1a65),
    DeclarationSeat::new(None, "GopassSecret", 50822, 0xd3e7931d8018b41f),
    DeclarationSeat::new(None, "FileSecret", 50981, 0xeb48cd9a89405069),
    DeclarationSeat::new(None, "SecretSource", 63670, 0x6f81a4283a4191c3),
    DeclarationSeat::new(None, "ProviderConfiguration", 30396, 0x291057a508d5dead),
    DeclarationSeat::new(None, "ConfigureProviderPayload", 38184, 0x9d1ca10f268001a7),
    DeclarationSeat::new(None, "RetireProviderPayload", 50924, 0x4b80f710a3a2c931),
    DeclarationSeat::new(None, "SetDefaultProviderPayload", 8793, 0xd1f67e850938a7cb),
    DeclarationSeat::new(None, "StartPayload", 40641, 0x2e0b1225b81453f5),
    DeclarationSeat::new(None, "StopPayload", 569, 0x04cb7084a6d2682f),
    DeclarationSeat::new(None, "ProviderConfiguredPayload", 17262, 0x916ed337ebe402f9),
    DeclarationSeat::new(None, "ProviderRetiredPayload", 13026, 0x7b415852ef5a66d3),
    DeclarationSeat::new(None, "DefaultProviderSetPayload", 24605, 0x1f04c4b9b85b9a3d),
    DeclarationSeat::new(None, "LifecycleState", 45702, 0x3df6a60f9c6807b7),
    DeclarationSeat::new(None, "Lifecycle", 58119, 0xf365eeeb32d91dc1),
    DeclarationSeat::new(None, "OrderRejectionReason", 15017, 0x6e870006e540eedb),
    DeclarationSeat::new(None, "OrderRejection", 33735, 0x34da1c0f5592d185),
    DeclarationSeat::new(None, "OperationKind", 18064, 0x7113c2d92d3d003f),
    DeclarationSeat::new(None, "UnimplementedReason", 1770, 0x98501b3f4c9b3989),
    DeclarationSeat::new(
        None,
        "RequestUnimplementedPayload",
        39160,
        0xfb3ea813d06a5fe3,
    ),
    DeclarationSeat::new(Some(28066), "ConfigureProvider", 37578, 0xee0397638b2519cd),
    DeclarationSeat::new(Some(28066), "RetireProvider", 46346, 0x0ca0e740727171c7),
    DeclarationSeat::new(Some(28066), "SetDefaultProvider", 43421, 0x9a657bcead077651),
    DeclarationSeat::new(Some(28066), "Start", 65166, 0xe938edfe73b8d9eb),
    DeclarationSeat::new(Some(28066), "Stop", 40136, 0x19d74aae68819315),
    DeclarationSeat::new(Some(30134), "ProviderConfigured", 30046, 0xb06730ac70c97c4f),
    DeclarationSeat::new(Some(30134), "ProviderRetired", 36216, 0x7b3d1f43143ef419),
    DeclarationSeat::new(Some(30134), "DefaultProviderSet", 16413, 0x3a6394bdeff27cf3),
    DeclarationSeat::new(Some(30134), "Started", 20083, 0x4608fece859b5d5d),
    DeclarationSeat::new(Some(30134), "Stopped", 31273, 0x498ff253a12d3fd7),
    DeclarationSeat::new(Some(30134), "OrderRejected", 47914, 0xea7a43af4b26d2e1),
    DeclarationSeat::new(
        Some(30134),
        "RequestUnimplemented",
        63693,
        0xe68a2a4ad44168fb,
    ),
    DeclarationSeat::new(Some(63670), "Environment", 22546, 0x4bdd4665876998a5),
    DeclarationSeat::new(Some(63670), "Gopass", 64442, 0x77ee3afb0928dc5f),
    DeclarationSeat::new(Some(63670), "File", 20532, 0x9af0648aefe832a9),
    DeclarationSeat::new(Some(63670), "NoSecret", 64695, 0x6b0da67ebeb3be03),
    DeclarationSeat::new(Some(45702), "Started", 9218, 0x64315e57b46764ed),
    DeclarationSeat::new(Some(45702), "Stopped", 4758, 0xfbb8d3eee36c71e7),
    DeclarationSeat::new(Some(15017), "ProviderUnknown", 40300, 0xd8c8f46252703371),
    DeclarationSeat::new(
        Some(15017),
        "ProviderAlreadyConfigured",
        39786,
        0xe3ceefa68cbb9c0b,
    ),
    DeclarationSeat::new(Some(15017), "EndpointInvalid", 56626, 0xa15e39209913e235),
    DeclarationSeat::new(Some(15017), "SecretUnavailable", 8695, 0xd5c427b6d84c206f),
    DeclarationSeat::new(
        Some(15017),
        "PolicyStoreUnavailable",
        38267,
        0x5302308c60eff539,
    ),
    DeclarationSeat::new(Some(18064), "ConfigureProvider", 45673, 0xc63b17691baf2313),
    DeclarationSeat::new(Some(18064), "RetireProvider", 50598, 0x2f29e46ebc72307d),
    DeclarationSeat::new(Some(18064), "SetDefaultProvider", 4681, 0x32904947c64007f7),
    DeclarationSeat::new(Some(18064), "Start", 35679, 0x9da7d178a45c9801),
    DeclarationSeat::new(Some(18064), "Stop", 36779, 0x489510e49948731b),
    DeclarationSeat::new(Some(1770), "NotInPrototypeScope", 56565, 0xeda090b864896fc5),
];
