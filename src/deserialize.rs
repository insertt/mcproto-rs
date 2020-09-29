use crate::types::VarInt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DeserializeErr {
    Eof,
    VarNumTooLong(Vec<u8>),
    NegativeLength(VarInt),
    BadStringEncoding(FromUtf8Error),
    InvalidBool(u8),
    NbtUnknownTagType(u8),
    NbtBadLength(isize),
    NbtInvalidStartTag(u8),
    CannotUnderstandValue(String),
    FailedJsonDeserialize(String)
}

impl<'b, R> Into<DeserializeResult<'b, R>> for DeserializeErr {
    #[inline]
    fn into(self) -> DeserializeResult<'b, R> {
        Err(self)
    }
}

pub struct Deserialized<'b, R> {
    pub value: R,
    pub data: &'b [u8],
}

impl<'b, R> Into<DeserializeResult<'b, R>> for Deserialized<'b, R> {
    #[inline]
    fn into(self) -> DeserializeResult<'b, R> {
        Ok(self)
    }
}

impl<'b, R> Deserialized<'b, R> {
    #[inline]
    pub fn create(value: R, data: &'b [u8]) -> Self {
        Deserialized {
            value,
            data,
        }
    }

    #[inline]
    pub fn ok(value: R, rest: &'b [u8]) -> DeserializeResult<'b, R> {
        Self::create(value, rest).into()
    }

    #[inline]
    pub fn replace<T>(self, other: T) -> Deserialized<'b, T> {
        Deserialized{
            value: other,
            data: self.data,
        }
    }

    #[inline]
    pub fn map<F, T>(self, f: F) -> Deserialized<'b, T> where F: FnOnce(R) -> T {
        Deserialized{
            value: f(self.value),
            data: self.data,
        }
    }

    #[inline]
    pub fn try_map<F, T>(self, f: F) -> DeserializeResult<'b, T> where
        F: FnOnce(R) -> Result<T, DeserializeErr>
    {
        match f(self.value) {
            Ok(new_value) => Ok(Deserialized{
                value: new_value,
                data: self.data,
            }),
            Err(err) => Err(err)
        }
    }

    #[inline]
    pub fn and_then<F, T>(self, f: F) -> DeserializeResult<'b, T> where
        F: FnOnce(R, &'b[u8]) -> DeserializeResult<'b, T>
    {
        f(self.value, self.data)
    }
}


impl<'b, R> From<(R, &'b [u8])> for Deserialized<'b, R> {
    fn from(v: (R, &'b [u8])) -> Self {
        let (value, data) = v;
        Deserialized {
            value,
            data,
        }
    }
}

pub type DeserializeResult<'b, R>
= Result<
    Deserialized<'b, R>,
    DeserializeErr>;

pub trait Deserialize: Sized {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<Self>;
}