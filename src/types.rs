// ... PRIMITIVE TYPES ...

use crate::*;
use crate::utils::*;
use crate::uuid::UUID4;

// bool
impl Serialize for bool {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte(if *self { 1 } else { 0 })
    }
}

impl Deserialize for bool {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        read_one_byte(data)?.try_map(move |b| {
            match b {
                0x00 => Ok(false),
                0x01 => Ok(true),
                other => Err(DeserializeErr::InvalidBool(other))
            }
        })
    }
}

// u8
impl Serialize for u8 {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte(*self)
    }
}

impl Deserialize for u8 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        read_one_byte(data)
    }
}

// i8
impl Serialize for i8 {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte(*self as u8)
    }
}

impl Deserialize for i8 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(read_one_byte(data)?.map(move |byte| byte as i8))
    }
}

// u16
impl Serialize for u16 {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let data = write_short(*self);
        to.serialize_bytes(&data[..])
    }
}

impl Deserialize for u16 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        read_short(data)
    }
}

// i16
impl Serialize for i16 {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        (*self as u16).mc_serialize(to)
    }
}

impl Deserialize for i16 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        u16::mc_deserialize(data)?.map(move |other| other as i16).into()
    }
}

// int
impl Serialize for i32 {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let data = write_int(*self as u32);
        to.serialize_bytes(&data[..])
    }
}

impl Deserialize for i32 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(read_int(data)?.map(move |v| v as i32))
    }
}

// long
impl Serialize for i64 {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let data = write_long(*self as u64);
        to.serialize_bytes(&data[..])
    }
}

impl Deserialize for i64 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(read_long(data)?.map(move |v| v as i64))
    }
}

// float
impl Serialize for f32 {

    //noinspection ALL
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let data = (*self).to_be_bytes();
        to.serialize_bytes(&data[..])
    }
}

impl Deserialize for f32 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        i32::mc_deserialize(data)?.map(move |r| f32::from_bits(r as u32)).into()
    }
}

// double
impl Serialize for f64 {
    //noinspection ALL
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let data = (*self).to_be_bytes();
        to.serialize_bytes(&data[..])
    }
}

impl Deserialize for f64 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        i64::mc_deserialize(data)?.map(move |r| f64::from_bits(r as u64)).into()
    }
}

// VAR INT AND VAR LONG
const VAR_INT_BYTES: usize = 5;
const VAR_LONG_BYTES: usize = 10;

const DESERIALIZE_VAR_INT: impl for<'b> Fn(&'b [u8]) -> DeserializeResult<'b, u64> = deserialize_var_num(VAR_INT_BYTES);
const DESERIALIZE_VAR_LONG: impl for<'b> Fn(&'b [u8]) -> DeserializeResult<'b, u64> = deserialize_var_num(VAR_LONG_BYTES);

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Default, Hash, Ord, Eq)]
pub struct VarInt(pub i32);

impl Serialize for VarInt {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let mut data = [0u8; VAR_INT_BYTES];
        to.serialize_bytes(serialize_var_num((self.0 as u32) as u64, &mut data))
    }
}

impl Deserialize for VarInt {
    fn mc_deserialize(orig_data: &[u8]) -> DeserializeResult<Self> {
        Ok(DESERIALIZE_VAR_INT(orig_data)?.map(move |v| VarInt(v as i32)))
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl From<i32> for VarInt {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for VarInt {
    fn from(v: usize) -> Self {
        Self(v as i32)
    }
}

impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VarInt({})", self.0)
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Default, Hash, Ord, Eq)]
pub struct VarLong(pub i64);

impl Serialize for VarLong {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let mut data = [0u8; VAR_LONG_BYTES];
        to.serialize_bytes(serialize_var_num(self.0 as u64, &mut data))
    }
}

impl Deserialize for VarLong {
    fn mc_deserialize(orig_data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(DESERIALIZE_VAR_LONG(orig_data)?.map(move |v| VarLong(v as i64)))
    }
}

fn serialize_var_num(data: u64, out: &mut [u8]) -> &[u8] {
    let mut v: u64 = data;
    let mut byte_idx = 0;
    let mut has_more = true;
    while has_more {
        if byte_idx == out.len() {
            panic!("tried to write too much data for Var num");
        }

        let mut v_byte = (v & 0x7F) as u8;
        v >>= 7;
        has_more = v != 0;
        if has_more {
            v_byte |= 0x80;
        }

        out[byte_idx] = v_byte;
        byte_idx += 1;
    }

    &out[..byte_idx]
}

const fn deserialize_var_num(max_bytes: usize) -> impl for<'b> Fn(&'b [u8]) -> DeserializeResult<'b, u64> {
    move |orig_data| {
        let mut data = orig_data;
        let mut v: u64 = 0;
        let mut bit_place: usize = 0;
        let mut i: usize = 0;
        let mut has_more = true;

        while has_more {
            if i == max_bytes {
                return DeserializeErr::VarNumTooLong(Vec::from(&orig_data[..i])).into();
            }
            let Deserialized { value: byte, data: rest } = read_one_byte(data)?;
            data = rest;
            has_more = byte & 0x80 != 0;
            v |= ((byte as u64) & 0x7F) << bit_place;
            bit_place += 7;
            i += 1;
        }

        Deserialized::ok(v, data)
    }
}

// STRING
impl Serialize for String {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_other(&VarInt(self.len() as i32))?;
        to.serialize_bytes(self.as_bytes())
    }
}

impl Deserialize for String {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        VarInt::mc_deserialize(data)?.and_then(move |length, rest| {
            if length.0 < 0 {
                Err(DeserializeErr::NegativeLength(length))
            } else {
                take(length.0 as usize)(rest)?.try_map(move |taken| {
                    String::from_utf8(taken.to_vec())
                        .map_err(DeserializeErr::BadStringEncoding)
                })
            }
        })
    }
}

// position
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub struct IntPosition {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Serialize for IntPosition {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let x_raw = if self.x < 0 {
            (self.x + 0x2000000) as u64 | 0x2000000
        } else {
            self.x as u64
        } & 0x3FFFFFF;
        let z_raw = if self.z < 0 {
            (self.z + 0x2000000) as u64 | 0x2000000
        } else {
            self.z as u64
        } & 0x3FFFFFF;
        let y_raw = if self.y < 0 {
            (self.y + 0x800) as u64 | 0x800
        } else {
            self.y as u64
        } & 0xFFF;

        let data_raw = ((x_raw << 38) | (z_raw << 12) | y_raw) as u64;
        let data_i64 = data_raw as i64;
        to.serialize_other(&data_i64)
    }
}

impl Deserialize for IntPosition {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized{ value: raw, data } = i64::mc_deserialize(data)?;
        let raw_unsigned = raw as u64;
        let mut x = ((raw_unsigned >> 38) as u32) & 0x3FFFFFF;
        let mut z = ((raw_unsigned >> 12) & 0x3FFFFFF) as u32;
        let mut y = ((raw_unsigned & 0xFFF) as u16) & 0xFFF;

        if (x & 0x2000000) != 0 { // is the 26th bit set
            // if so, treat the rest as a positive integer, and treat 26th bit as -2^25
            // 2^25 == 0x2000000
            // 0x1FFFFFF == 2^26 - 1 (all places set to 1 except 26th place)
            x = (((x & 0x1FFFFFF) as i32) - 0x2000000) as u32;
        }
        if (y & 0x800) != 0 {
            y = (((y & 0x7FF) as i16) - 0x800) as u16;
        }
        if (z & 0x2000000) != 0 {
            z = (((z & 0x1FFFFFF) as i32) - 0x2000000) as u32;
        }

        Deserialized::ok(IntPosition{
            x: x as i32,
            y: y as i16,
            z: z as i32
        }, data)
    }
}

// angle
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Angle {
    pub value: u8
}

impl Serialize for Angle {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte(self.value)
    }
}

impl Deserialize for Angle {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(read_one_byte(data)?.map(move |b| {
            Angle { value: b }
        }))
    }
}

// UUID

impl Serialize for UUID4 {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let bytes = self.to_u128().to_be_bytes();
        to.serialize_bytes(&bytes[..])
    }
}

impl Deserialize for UUID4 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        take(16)(data)?.map(move |bytes| {
            let raw = (bytes[0] as u128) << 120 |
                (bytes[1] as u128) << 112 |
                (bytes[2] as u128) << 104 |
                (bytes[3] as u128) << 96 |
                (bytes[4] as u128) << 88 |
                (bytes[5] as u128) << 80 |
                (bytes[6] as u128) << 72 |
                (bytes[7] as u128) << 64 |
                (bytes[8] as u128) << 56 |
                (bytes[9] as u128) << 48 |
                (bytes[10] as u128) << 40 |
                (bytes[11] as u128) << 32 |
                (bytes[12] as u128) << 24 |
                (bytes[13] as u128) << 16 |
                (bytes[14] as u128) << 8 |
                bytes[15] as u128;
            UUID4::from(raw)
        }).into()
    }
}

// NBT

#[derive(Clone, PartialEq, Debug)]
pub struct NamedNbtTag {
    pub root: nbt::NamedTag
}

impl Serialize for NamedNbtTag {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let bytes = self.root.bytes();
        to.serialize_bytes(bytes.as_slice())
    }
}

impl Deserialize for NamedNbtTag {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(nbt::NamedTag::root_compound_tag_from_bytes(data)?.map(move |root| NamedNbtTag { root }))
    }
}

impl From<nbt::NamedTag> for NamedNbtTag {
    fn from(root: nbt::NamedTag) -> Self {
        Self { root }
    }
}

impl Into<nbt::NamedTag> for NamedNbtTag {
    fn into(self) -> nbt::NamedTag {
        self.root
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedInt {
    raw: i32
}

impl Serialize for FixedInt {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_other(&self.raw)
    }
}

impl Deserialize for FixedInt {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(i32::mc_deserialize(data)?.map(move |raw| {
            FixedInt{ raw }
        }))
    }
}

impl FixedInt {
    pub fn new(data: f64, fractional_bytes: usize) -> Self {
        Self { raw: (data * ((1 << fractional_bytes) as f64)) as i32 }
    }

    pub fn into_float(self, fractional_bytes: usize) -> f64 {
        (self.raw as f64) / ((1 << fractional_bytes) as f64)
    }
}

// chat
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Chat {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<Chat>>
}

impl ToString for Chat {
    fn to_string(&self) -> String {
        self.extra.as_ref()
            .into_iter()
            .flat_map(|v| v.into_iter())
            .map(|item| item.to_string())
            .fold(self.text.clone(), |acc, v| acc + v.as_str())
    }
}

const SECTION_SYMBOL: char = '§';

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum ColorCode {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White
}

impl ColorCode {
    pub fn from_code(i: &char) -> Option<Self> {
        match i {
            '0' => Some(ColorCode::Black),
            '1' => Some(ColorCode::DarkBlue),
            '2' => Some(ColorCode::DarkGreen),
            '3' => Some(ColorCode::DarkAqua),
            '4' => Some(ColorCode::DarkRed),
            '5' => Some(ColorCode::DarkPurple),
            '6' => Some(ColorCode::Gold),
            '7' => Some(ColorCode::Gray),
            '8' => Some(ColorCode::DarkGray),
            '9' => Some(ColorCode::Blue),
            'a' => Some(ColorCode::Green),
            'b' => Some(ColorCode::Aqua),
            'c' => Some(ColorCode::Red),
            'd' => Some(ColorCode::LightPurple),
            'e' => Some(ColorCode::Yellow),
            'f' => Some(ColorCode::White),
            _ => None
        }
    }

    pub fn to_code(&self) -> char {
        match self {
            ColorCode::Black => '0',
            ColorCode::DarkBlue => '1',
            ColorCode::DarkGreen => '2',
            ColorCode::DarkAqua => '3',
            ColorCode::DarkRed => '4',
            ColorCode::DarkPurple => '5',
            ColorCode::Gold => '6',
            ColorCode::Gray => '7',
            ColorCode::DarkGray => '8',
            ColorCode::Blue => '9',
            ColorCode::Green => 'a',
            ColorCode::Aqua => 'b',
            ColorCode::Red => 'c',
            ColorCode::LightPurple => 'd',
            ColorCode::Yellow => 'e',
            ColorCode::White => 'f',
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "black" => Some(ColorCode::Black),
            "dark_blue" => Some(ColorCode::DarkBlue),
            "dark_green" => Some(ColorCode::DarkGreen),
            "dark_aqua" => Some(ColorCode::DarkAqua),
            "dark_red" => Some(ColorCode::DarkRed),
            "dark_purple" => Some(ColorCode::DarkPurple),
            "gold" => Some(ColorCode::Gold),
            "gray" => Some(ColorCode::Gray),
            "dark_gray" => Some(ColorCode::DarkGray),
            "blue" => Some(ColorCode::Blue),
            "green" => Some(ColorCode::Green),
            "aqua" => Some(ColorCode::Aqua),
            "red" => Some(ColorCode::Red),
            "light_purple" => Some(ColorCode::LightPurple),
            "yellow" => Some(ColorCode::Yellow),
            "white" => Some(ColorCode::White),
            _ => None
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ColorCode::Black => "black",
            ColorCode::DarkBlue => "dark_blue",
            ColorCode::DarkGreen => "dark_green",
            ColorCode::DarkAqua => "dark_aqua",
            ColorCode::DarkRed => "dark_red",
            ColorCode::DarkPurple => "dark_purple",
            ColorCode::Gold => "gold",
            ColorCode::Gray => "gray",
            ColorCode::DarkGray => "dark_gray",
            ColorCode::Blue => "blue",
            ColorCode::Green => "green",
            ColorCode::Aqua => "aqua",
            ColorCode::Red => "red",
            ColorCode::LightPurple => "light_purple",
            ColorCode::Yellow => "yellow",
            ColorCode::White => "white",
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub enum Formatter {
    Color(ColorCode),
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic,
    Reset
}

impl Formatter {
    pub fn from_code(i: &char) -> Option<Self> {
        match i.to_ascii_lowercase() {
            'k' => Some(Formatter::Obfuscated),
            'l' => Some(Formatter::Bold),
            'm' => Some(Formatter::Strikethrough),
            'n' => Some(Formatter::Underline),
            'o' => Some(Formatter::Italic),
            'r' => Some(Formatter::Reset),
            _ => ColorCode::from_code(i).map(Formatter::Color)
        }
    }

    pub fn code(&self) -> char {
        match self {
            Formatter::Color(c) => c.to_code(),
            Formatter::Obfuscated => 'k',
            Formatter::Bold => 'l',
            Formatter::Strikethrough => 'm',
            Formatter::Underline => 'n',
            Formatter::Italic => 'o',
            Formatter::Reset => 'r'
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "obfuscated" => Some(Formatter::Obfuscated),
            "bold" => Some(Formatter::Bold),
            "strikethrough" => Some(Formatter::Strikethrough),
            "underline" => Some(Formatter::Underline),
            "italic" => Some(Formatter::Italic),
            "reset" => Some(Formatter::Reset),
            _ => ColorCode::from_name(name).map(Formatter::Color)
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Formatter::Obfuscated => "obfuscated",
            Formatter::Bold => "bold",
            Formatter::Strikethrough => "strikethrough",
            Formatter::Underline => "underline",
            Formatter::Italic => "italic",
            Formatter::Reset => "reset",
            Formatter::Color(c) => c.name(),
        }
    }
}

impl ToString for Formatter {
    fn to_string(&self) -> String {
        vec!(SECTION_SYMBOL, self.code()).into_iter().collect()
    }
}

impl Chat {
    pub fn to_traditional(&self) -> String {
        self.to_traditional_parts(Vec::<Formatter>::new().as_ref(), None)
    }

    fn to_traditional_parts(&self, formatters: &Vec<Formatter>, color: Option<ColorCode>) -> String {
        let mut own_formatters = formatters.clone();
        Self::update_formatter(&mut own_formatters, Formatter::Bold, &self.bold);
        Self::update_formatter(&mut own_formatters, Formatter::Italic, &self.italic);
        Self::update_formatter(&mut own_formatters, Formatter::Underline, &self.underlined);
        Self::update_formatter(&mut own_formatters, Formatter::Strikethrough, &self.strikethrough);
        Self::update_formatter(&mut own_formatters, Formatter::Obfuscated, &self.obfuscated);

        let own_color_option = self.color.as_ref()
            .map(String::as_str)
            .and_then(ColorCode::from_name)
            .or(color);

        let own_color = own_color_option
            .map(Formatter::Color)
            .map(|f| f.to_string());

        let own_formatter =
            own_formatters
                .clone()
                .into_iter()
                .map(|f| f.to_string())
                .fold(String::new(), |acc, v| acc + v.as_str());

        let own_color_str = match own_color {
            Some(v) => v,
            None => String::new()
        };

        let own_out = own_formatter +  own_color_str.as_str() + self.text.as_str();

        self.extra.as_ref()
            .into_iter()
            .flat_map(|v| v.into_iter())
            .map(|child| child.to_traditional_parts(&own_formatters, own_color_option))
            .fold(own_out, |acc, next| acc + next.as_str())
    }

    fn update_formatter(to: &mut Vec<Formatter>, formatter: Formatter, v: &Option<bool>) {
        if !to.contains(&formatter) && v.unwrap_or(false) {
            to.push(formatter)
        }
    }
}

impl Serialize for Chat {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        serde_json::to_string(self).map_err(move |err| {
            SerializeErr::FailedJsonEncode(format!("failed to serialize chat {:?}", err))
        })?.mc_serialize(to)
    }
}

impl Deserialize for Chat {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        String::mc_deserialize(data)?.try_map(move |str| {
            serde_json::from_str(str.as_str()).map_err(move |err| {
                DeserializeErr::FailedJsonDeserialize(format!("failed to deserialize chat {:?}", err))
            })
        })
    }
}

#[derive(Default)]
pub struct BytesSerializer {
    data: Vec<u8>
}

impl Serializer for BytesSerializer {
    fn serialize_bytes(&mut self, data: &[u8]) -> SerializeResult {
        self.data.extend_from_slice(data);
        Ok(())
    }
}

impl BytesSerializer {
    pub fn with_capacity(cap: usize) -> Self {
        BytesSerializer{
            data: Vec::with_capacity(cap),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }
}

impl<T> Serialize for Option<T> where T: Serialize {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        match self {
            Some(value) => {
                to.serialize_other(&true)?;
                to.serialize_other(value)
            },
            None => {
                to.serialize_other(&false)
            }
        }
    }
}

impl<T> Deserialize for Option<T> where T: Deserialize {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        bool::mc_deserialize(data)?.and_then(move |is_present, data| {
            if is_present {
                Ok(T::mc_deserialize(data)?.map(move |component| Some(component)))
            } else {
                Deserialized::ok(None, data)
            }
        })
    }
}

// SLOT
#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    pub item_id: VarInt,
    pub item_count: i8,
    pub nbt: Option<nbt::NamedTag>,
}

impl Serialize for Slot {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_other(&self.item_id)?;
        to.serialize_other(&self.item_count)?;
        match self.nbt.as_ref() {
            Some(nbt) => to.serialize_bytes(nbt.bytes().as_slice()),
            None => to.serialize_byte(nbt::Tag::End.id()),
        }
    }
}

impl Deserialize for Slot {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized{ value: item_id, data } = VarInt::mc_deserialize(data)?;
        let Deserialized{ value: item_count, data } = i8::mc_deserialize(data)?;
        if data.is_empty() {
            return Err(DeserializeErr::Eof);
        }

        let id = data[0];
        let rest = &data[1..];
        Ok(match id {
            0x00 => Deserialized{ value: None, data: rest },
            _ => nbt::read_named_tag(data)?.map(move |tag| Some(tag))
        }.map(move |nbt| {
            Slot{ item_id, item_count, nbt }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn test_bool() {
        test_type(true);
        test_type(false);
    }

    #[test]
    fn test_signed_byte() {
        test_type(0i8);
        test_type(127i8);
        test_type(-15i8);
    }

    #[test]
    fn test_unsigned_byte() {
        test_type(0u8);
        test_type(128u8);
        test_type(255u8);
    }

    #[test]
    fn test_signed_short() {
        test_type(0i16);
        test_type(-88i16);
        test_type(25521i16);
    }

    #[test]
    fn test_unsigned_short() {
        test_type(0u16);
        test_type(1723u16);
        test_type(65534u16);
    }

    #[test]
    fn test_signed_int() {
        test_type(0i32);
        test_type(123127i32);
        test_type(-171238i32);
        test_type(2147483647i32);
    }

    #[test]
    fn test_signed_long() {
        test_type(0i64);
        test_type(123127i64);
        test_type(-12123127i64);
        test_type(2147483647i64);
        test_type(-10170482028482i64);
    }

    #[test]
    fn test_float() {
        test_type(0.2313f32);
        test_type(0f32);
        test_type(123123213f32);
        test_type(-123123f32);
    }

    #[test]
    fn test_double() {
        test_type(0.2313f64);
        test_type(0f64);
        test_type(123123213f64);
        test_type(-123123f64);
    }

    #[test]
    fn test_var_int() {
        test_type(VarInt(0));
        test_type(VarInt(1231231));
        test_type(VarInt(2147483647));
        test_type(VarInt(-2147483648));
        test_type(VarInt(-1));
        test_type(VarInt(-1001237));
    }

    #[test]
    fn test_var_long() {
        test_type(VarLong(0));
        test_type(VarLong(1231231));
        test_type(VarLong(12312319123));
        test_type(VarLong(9223372036854775807));
        test_type(VarLong(-1));
        test_type(VarLong(-12312319123));
        test_type(VarLong(-9223372036854775808));
        test_type(VarLong(-1001237));
    }

    #[test]
    fn test_string() {
        test_type(String::from("hello my name is joey 123"));
        test_type(String::from(""));
        test_type(String::from("AAAA"));
        test_type(String::from("hello my name is joey 123").repeat(1000));
    }

    #[test]
    fn test_nbt() {
        test_type(NamedNbtTag {root: nbt::Tag::Compound(vec!(
            nbt::Tag::String("test 123".to_owned()).with_name("abc 123")
        )).with_name("root")})
    }

    #[test]
    fn test_int_position() {
        test_type(IntPosition{
            x: 12312,
            y: -32,
            z: 321312,
        });

        test_type(IntPosition{
            x: 12312,
            y: -32,
            z: -321312,
        });

        test_type(IntPosition{
            x: -12312,
            y: -32,
            z: -321312,
        });

        test_type(IntPosition{
            x: -12312,
            y: 32,
            z: 321312,
        });

        test_type(IntPosition{
            x: 0,
            y: 0,
            z: 0,
        });

        test_type(IntPosition{
            x: 48,
            y: 232,
            z: 12,
        });

        test_type(IntPosition{
            x: 33554431,
            y: 2047,
            z: 33554431,
        });

        test_type(IntPosition{
            x: -33554432,
            y: -2048,
            z: -33554432,
        });

        test_type(IntPosition{
            x: 3,
            y: 0,
            z: 110655,
        });
    }

    #[test]
    fn test_uuid() {
        for _ in 0..5 {
            test_type(UUID4::random());
        }
    }

    #[test]
    fn test_angle() {
        test_type(Angle{
            value: 0,
        });
        test_type(Angle{
            value: 24,
        });
        test_type(Angle{
            value: 255,
        });
        test_type(Angle{
            value: 8,
        });
    }

    fn test_type<S: Serialize + Deserialize + PartialEq + Debug>(value: S) {
        let bytes = {
            let mut test = BytesSerializer::default();
            value.mc_serialize(&mut test).expect("serialization should succeed");
            test.into_bytes()
        };
        let deserialized = S::mc_deserialize(bytes.as_slice()).expect("deserialization should succeed");
        assert!(deserialized.data.is_empty());
        assert_eq!(deserialized.value, value, "deserialized value == serialized value");
        let re_serialized = {
            let mut test = BytesSerializer::default();
            deserialized.value.mc_serialize(&mut test).expect("serialization should succeed");
            test.into_bytes()
        };
        assert_eq!(re_serialized, bytes, "serialized value == original serialized bytes");
    }
}