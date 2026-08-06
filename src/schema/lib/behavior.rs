// Handwritten operational behavior for the authority-verified ordinary Mirror Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, readable Dotos roles, and the allocated
// Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};
use signal_standard::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 {
        return Err(WireShapeError);
    }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self {
                Self(payload)
            }
            pub fn payload(&self) -> &$inner {
                &self.0
            }
            pub fn into_payload(self) -> $inner {
                self.0
            }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                self.0.to_wire()
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                Ok(Self(<$inner as WireShape>::from_wire(value)?))
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_enum!(z2VU7B { unit {  } unary { 0 => z2VZYM(z2VauH) : "RetireProvider", 1 => z2VXhH(z2VKw6) : "Stop", 2 => z2VYfv(z2VNNt) : "SetDefaultProvider", 3 => z2Vf8q(z2VXqz) : "Start", 4 => z2VWwB(z2VX7d) : "ConfigureProvider" } });
wire_enum!(z2VUiq { unit {  } unary { 0 => z2VQeG(z2VT5W) : "DefaultProviderSet", 1 => z2VRjY(z2Vd3L) : "Started", 2 => z2VV4U(z2Vd3L) : "Stopped", 3 => z2VWXh(z2VPds) : "ProviderRetired", 4 => z2VUhK(z2VQtu) : "ProviderConfigured", 5 => z2VehS(z2VXQT) : "RequestUnimplemented", 6 => z2Va1P(z2VVnv) : "OrderRejected" } });
wire_external_newtype!(z2Vdno, std::string::String);
wire_external_newtype!(z2VQu1, std::string::String);
wire_external_newtype!(z2VQyu, std::string::String);
wire_external_newtype!(z2VMcF, std::string::String);
wire_external_newtype!(z2VTH3, std::string::String);
wire_external_newtype!(z2Vb98, std::string::String);
wire_external_newtype!(z2VaXy, std::string::String);
wire_newtype!(z2VTjG, z2VMcF);
wire_newtype!(z2VasX, z2VTH3);
wire_newtype!(z2VavG, z2Vb98);
wire_enum!(z2Veh3 { unit { 1 => z2Vezi : "NoSecret" } unary { 0 => z2VSU1(z2VTjG) : "Environment", 2 => z2VevM(z2VasX) : "Gopass", 3 => z2VRsH(z2VavG) : "File" } });
wire_struct!(z2VUoM { field_0: z2Vdno, field_1: z2VQu1, field_2: z2VQyu, field_3: z2Veh3 });
wire_newtype!(z2VX7d, z2VUoM);
wire_newtype!(z2VauH, z2Vdno);
wire_newtype!(z2VNNt, z2Vdno);
wire_struct!(z2VXqz {  });
wire_struct!(z2VKw6 {  });
wire_newtype!(z2VQtu, z2Vdno);
wire_newtype!(z2VPds, z2Vdno);
wire_newtype!(z2VT5W, z2Vdno);
wire_enum!(z2VZMF { unit { 0 => z2VNWD : "Started", 1 => z2VMBK : "Stopped" } unary {  } });
wire_newtype!(z2Vd3L, z2VZMF);
wire_enum!(z2VQEC { unit { 0 => z2VX94 : "PolicyStoreUnavailable", 1 => z2Vcbb : "EndpointInvalid", 2 => z2VNMC : "SecretUnavailable", 3 => z2VXk7 : "ProviderUnknown", 4 => z2VXbF : "ProviderAlreadyConfigured" } unary {  } });
wire_struct!(z2VVnv { field_0: z2VQEC, field_1: z2VaXy });
wire_enum!(z2VR8j { unit { 0 => z2Vaof : "RetireProvider", 1 => z2VM9z : "SetDefaultProvider", 2 => z2VWhQ : "Stop", 3 => z2VWNS : "Start", 4 => z2VZLk : "ConfigureProvider" } unary {  } });
wire_enum!(z2VLHo { unit { 0 => z2VcaY : "NotInPrototypeScope" } unary {  } });
wire_struct!(z2VXQT { field_0: z2VR8j, field_1: z2VLHo });

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer>
            for signal_standard::schema::lib::ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            signal_standard::schema::lib::ArchivedWireValue:
                RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <signal_standard::schema::lib::ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VU7B);
archive_root!(z2VUiq);
archive_root!(z2Vdno);
archive_root!(z2VQu1);
archive_root!(z2VQyu);
archive_root!(z2VMcF);
archive_root!(z2VTH3);
archive_root!(z2Vb98);
archive_root!(z2VaXy);
archive_root!(z2VTjG);
archive_root!(z2VasX);
archive_root!(z2VavG);
archive_root!(z2Veh3);
archive_root!(z2VUoM);
archive_root!(z2VX7d);
archive_root!(z2VauH);
archive_root!(z2VNNt);
archive_root!(z2VXqz);
archive_root!(z2VKw6);
archive_root!(z2VQtu);
archive_root!(z2VPds);
archive_root!(z2VT5W);
archive_root!(z2VZMF);
archive_root!(z2Vd3L);
archive_root!(z2VQEC);
archive_root!(z2VVnv);
archive_root!(z2VR8j);
archive_root!(z2VLHo);
archive_root!(z2VXQT);


pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(16) { Ok(value) => value, Err(_) => panic!("contract ID is allocated") },
        match signal_frame::WireRevision::try_new(2) { Ok(value) => value, Err(_) => panic!("wire revision is allocated") },
    );
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")]
    FrameEncode,
    #[error("failed to decode bound signal frame")]
    ArchiveDecode,
    #[error("unexpected signal frame body")]
    UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")]
    OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute { RetireProvider, Stop, SetDefaultProvider, Start, ConfigureProvider }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute { DefaultProviderSet, Started, Stopped, ProviderRetired, ProviderConfigured, RequestUnimplemented, OrderRejected }

impl z2VU7B {
    pub fn route(&self) -> InputRoute { match self { Self::z2VZYM(_) => InputRoute::RetireProvider, Self::z2VXhH(_) => InputRoute::Stop, Self::z2VYfv(_) => InputRoute::SetDefaultProvider, Self::z2Vf8q(_) => InputRoute::Start, Self::z2VWwB(_) => InputRoute::ConfigureProvider, } }
    pub fn wire_route(&self) -> signal_frame::WireRoute { signal_frame::WireRoute::new(signal_frame::RootCode::new(0), signal_frame::VariantCode::new(self.route() as u8)) }
    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame { let route = self.wire_route(); Frame::new(route, FrameBody::Request { exchange, request: signal_frame::Request::from_payload(self) }) }
    pub fn encode_request_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> { self.into_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode) }
}

impl z2VUiq {
    pub fn route(&self) -> OutputRoute { match self { Self::z2VQeG(_) => OutputRoute::DefaultProviderSet, Self::z2VRjY(_) => OutputRoute::Started, Self::z2VV4U(_) => OutputRoute::Stopped, Self::z2VWXh(_) => OutputRoute::ProviderRetired, Self::z2VUhK(_) => OutputRoute::ProviderConfigured, Self::z2VehS(_) => OutputRoute::RequestUnimplemented, Self::z2Va1P(_) => OutputRoute::OrderRejected, } }
    pub fn wire_route(&self) -> signal_frame::WireRoute { signal_frame::WireRoute::new(signal_frame::RootCode::new(1), signal_frame::VariantCode::new(self.route() as u8)) }
    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame { let route = self.wire_route(); let reply = signal_frame::Reply::committed(signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self))); Frame::new(route, FrameBody::Reply { exchange, reply }) }
    pub fn encode_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> { self.into_reply_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode) }
}

impl signal_frame::RequestPayload for z2VU7B {}
impl signal_frame::SignalOperationHeads for z2VU7B { const HEADS: &'static [&'static str] = &["RetireProvider", "Stop", "SetDefaultProvider", "Start", "ConfigureProvider"]; }
impl signal_frame::LogVariant for z2VU7B { fn log_variant(&self) -> u64 { let route = self.wire_route(); u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8) } }

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VU7B, z2VUiq>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VU7B, z2VUiq>;
pub type Request = signal_frame::Request<z2VU7B>;
pub type ReplyEnvelope = signal_frame::Reply<z2VUiq>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VU7B>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> { Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode) }
    pub fn decode_single_request(bytes: &[u8]) -> Result<(signal_frame::ExchangeIdentifier, z2VU7B), SignalFrameError> { match Self::decode_frame(bytes)?.into_body() { FrameBody::Request { exchange, request } => { let found = request.payloads().len(); if found != 1 { return Err(SignalFrameError::OperationCount { found }); } Ok((exchange, request.payloads.into_head())) }, _ => Err(SignalFrameError::UnexpectedFrameBody) } }
}

