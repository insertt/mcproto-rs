use crate::{Serialize, Deserialize, DeserializeErr};
use std::fmt::Debug;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProtocolSpec {
    pub name: String,
    pub packets: Vec<ProtocolPacketSpec>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProtocolPacketSpec {
    pub state: String,
    pub direction: String,
    pub id: i32,
    pub name: String,
    pub fields: Vec<ProtocolPacketField>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProtocolPacketField {
    pub name: String,
    pub kind: String,
}

pub trait PacketIdentifier: Clone + Debug + PartialEq + Serialize {}

impl<T: Clone + Debug + PartialEq + Serialize> PacketIdentifier for T {}

pub trait Packet<I: PacketIdentifier>: Serialize {
    fn id(&self) -> I;

    fn mc_deserialize(raw: RawPacket<I>) -> Result<Self, PacketErr>;
}

#[derive(Debug)]
pub enum PacketErr {
    UnknownId(i32),
    DeserializeFailed(DeserializeErr)
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawPacket<I> {
    pub id: I,
    pub data: Vec<u8>,
}

pub trait ProtocolType: Serialize + Deserialize {}

impl<T: Serialize + Deserialize> ProtocolType for T {}

#[macro_export]
macro_rules! as_item {
    ($i:item) => { $i };
}

#[macro_export]
macro_rules! __protocol_body_serialize_def_helper {
    ($to: ident, $slf: ident, $fieldn: ident, $($field_rest: ident),+) => {
        $to.serialize_other(&$slf.$fieldn)?;
        $crate::__protocol_body_serialize_def_helper!($to, $slf, $($field_rest),+);
    };

    ( $to: ident, $slf: ident, $fieldn: ident ) => {
        $to.serialize_other(&$slf.$fieldn)
    };
}

#[macro_export]
macro_rules! __protocol_body_def_helper {
    ($bodyt: ident { }) => {
        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct $bodyt;

        impl Serialize for $bodyt {
            fn mc_serialize<S: Serializer>(&self, _: &mut S) -> SerializeResult {
                Ok(())
            }
        }

        impl Deserialize for $bodyt {
            fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
                Deserialized::ok(Self::default(), data)
            }
        }
    };
    ($bodyt: ident { $($fname: ident: $ftyp: ty ),+ }) => {
        $crate::as_item! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct $bodyt {
               $(pub $fname: $ftyp),+
            }
        }

        impl Serialize for $bodyt {
            fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
                $(
                    to.serialize_other(&self.$fname)?;
                )+
                Ok(())
            }
        }

        impl Deserialize for $bodyt {
            fn mc_deserialize(_rest: &[u8]) -> DeserializeResult<'_, Self> {
                $(let Deserialized{ value: $fname, data: _rest } = <$ftyp>::mc_deserialize(_rest)?;)+

                Deserialized::ok(Self{ $($fname),+ }, _rest)
            }
        }
    }
}

#[macro_export]
macro_rules! define_protocol {
    ($packett: ident, $directiont: ident, $statet: ident, $idt: ident, $idi: ident => { $($nam: ident, $id: literal, $state: ident, $direction: ident => $body: ident { $($fnam: ident: $ftyp: ty),* }),*}) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $idi {
            pub id: $idt,
            pub state: $statet,
            pub direction: $directiont
        }

        impl crate::Serialize for $idi {
            fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
                VarInt(self.id).mc_serialize(to)
            }
        }

        impl From<($idt, $statet, $directiont)> for $idi {
            fn from(tuple: ($idt, $statet, $directiont)) -> Self {
                let (id, state, direction) = tuple;
                Id { id, state, direction }
            }
        }

        $crate::as_item! {
            #[derive(Debug, PartialEq, Clone)]
            pub enum $packett {
                $($nam($body)),*,
            }
        }

        impl crate::protocol::Packet<$idi> for $packett {
            fn id(&self) -> $idi {
                use self::$packett::*;
                use self::$statet::*;
                use self::$directiont::*;

                match self {
                    $($nam(_) => ($id, $state, $direction)),*
                }.into()
            }

            fn mc_deserialize(raw: crate::protocol::RawPacket<$idi>) ->
                Result<Self, crate::protocol::PacketErr>
            {
                use self::$packett::*;
                use self::$statet::*;
                use self::$directiont::*;
                use crate::protocol::PacketErr::*;
                use crate::Deserialize;

                let id = raw.id;
                let data = raw.data.as_slice();

                match (id.id, id.state, id.direction) {
                    $(($id, $state, $direction) => Ok($nam($body::mc_deserialize(data).map_err(DeserializeFailed)?.value))),*,
                    other => Err(UnknownId(other.0)),
                }
            }
        }

        impl crate::Serialize for $packett {
            fn mc_serialize<S: crate::Serializer>(&self, to: &mut S) -> crate::SerializeResult {
                use self::$packett::*;
                match self {
                    $($nam(body) => to.serialize_other(body)),+
                }
            }
        }

        impl $packett {
            pub fn describe() -> crate::protocol::ProtocolSpec {
                crate::protocol::ProtocolSpec {
                    name: stringify!($packett).to_owned(),
                    packets: vec!(
                        $(crate::protocol::ProtocolPacketSpec{
                            state: stringify!($state).to_owned(),
                            direction: stringify!($direction).to_owned(),
                            id: $id,
                            name: stringify!($nam).to_owned(),
                            fields: vec!(
                                $(crate::protocol::ProtocolPacketField{
                                    name: stringify!($fnam).to_owned(),
                                    kind: stringify!($ftyp).to_owned(),
                                }),*
                            )
                        }),*,
                    )
                }
            }
        }

        $($crate::__protocol_body_def_helper!($body { $($fnam: $ftyp),* });)*
    };
}

#[macro_export]
macro_rules! proto_enum_with_type {
    ($typ: ty, $from_nam: ident, $as_nam: ident, $fmt: literal, $typname: ident, $(($bval: literal, $nam: ident)),*) => {
        $crate::as_item! {
            #[derive(PartialEq, Clone, Copy)]
            pub enum $typname {
                $($nam),*
            }
        }

        impl Serialize for $typname {
            fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
                to.serialize_other(&self.$as_nam())
            }
        }

        impl Deserialize for $typname {
            fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
                <$typ>::mc_deserialize(data)?.and_then(move |id, rest| {
                    Self::$from_nam(id).map(move |val| {
                        Deserialized::ok(val, rest)
                    }).unwrap_or_else(|| Err(DeserializeErr::CannotUnderstandValue(format!("invalid {} {}", stringify!($typname), id))))
                })
            }
        }

        impl Into<$typ> for $typname {
            fn into(self) -> $typ {
                use $typname::*;
                match self {
                    $($nam => $bval.into()),*,
                }
            }
        }

        impl std::fmt::Display for $typname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $fmt, self.name(), self.$as_nam())?;
                Ok(())
            }
        }

        impl std::fmt::Debug for $typname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $fmt, self.name(), self.$as_nam())?;
                Ok(())
            }
        }

        impl $typname {
            pub fn $from_nam(b: $typ) -> Option<Self> {
                use $typname::*;
                match b.into() {
                    $($bval => Some($nam)),*,
                    _ => None
                }
            }

            pub fn name(&self) -> &str {
                use $typname::*;
                match self {
                    $($nam => stringify!($nam)),+,
                }
            }

            pub fn $as_nam(&self) -> $typ {
                (*self).into()
            }
        }
    }
}

#[macro_export]
macro_rules! proto_byte_enum {
    ($typname: ident, $($bval: literal :: $nam: ident),*) => {
        proto_enum_with_type!(u8, from_byte, as_byte, "{}(0x{:02x})", $typname, $(($bval, $nam)),*);
    }
}

#[macro_export]
macro_rules! proto_varint_enum {
    ($typname: ident, $($bval: literal :: $nam: ident),*) => {
        proto_enum_with_type!(VarInt, from_varint, as_varint, "{}({:?})", $typname, $(($bval, $nam)),*);
    }
}

#[macro_export]
macro_rules! proto_int_enum {
    ($typname: ident, $($bval: literal :: $nam: ident),*) => {
        proto_enum_with_type!(i32, from_int, as_int, "{}(0x{:02x})", $typname, $(($bval, $nam)),*);
    }
}

#[macro_export]
macro_rules! proto_byte_flag {
    ($typname: ident, $($bval: literal :: $nam: ident),*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
        pub struct $typname(pub u8);

        impl $typname {
            $(paste::paste! {
                pub fn [<is_ $nam>](&self) -> bool {
                    self.0 & $bval != 0
                }
            })*

            $(paste::paste! {
                pub fn [<set_ $nam>](&mut self, value: bool) {
                    if value {
                        self.0 |= $bval;
                    } else {
                        self.0 ^= $bval;
                    }
                }
            })*

            $(paste::paste! {
                pub fn [<with_ $nam>](mut self, value: bool) -> Self {
                    if value {
                        self.0 |= $bval;
                    } else {
                        self.0 ^= $bval;
                    }

                    self
                }
            })*
        }

        impl Serialize for $typname {
            fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
                to.serialize_byte(self.0)
            }
        }

        impl Deserialize for $typname {
            fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
                Ok(u8::mc_deserialize(data)?.map(move |b| $typname(b)))
            }
        }
    }
}