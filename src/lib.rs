//! NBT (Named Binary Tag) is a tag based binary format designed to carry large
//! amounts of binary data with smaller amounts of additional data.
//!
//! # Examples
//!
//! ## Read
//!
//! ```
//! use nbt::decode::read_compound_tag;
//! use std::io::Cursor;
//!
//! let mut cursor = Cursor::new(include_bytes!("../test/binary/servers.dat").to_vec());
//! let root_tag = read_compound_tag(&mut cursor).unwrap();
//!
//! let servers = root_tag.get_compound_tag_vec("servers").unwrap();
//! assert_eq!(servers.len(), 1);
//!
//! let server = servers[0];
//! let ip = server.get_str("ip").unwrap();
//! let name = server.get_str("name").unwrap();
//! let hide_address = server.get_bool("hideAddress").unwrap();
//!
//! assert_eq!(ip, "localhost:25565");
//! assert_eq!(name, "Minecraft Server");
//! assert!(hide_address);
//! ```
//!
//! ## Write
//!
//! ```
//! use nbt::encode::write_compound_tag;
//! use nbt::CompoundTag;
//!
//! let mut server = CompoundTag::new();
//!
//! server.insert_str("ip", "localhost:25565");
//! server.insert_str("name", "Minecraft Server");
//! server.insert_bool("hideAddress", true);
//!
//! let mut servers = Vec::new();
//! servers.push(server);
//!
//! let mut root_tag = CompoundTag::new();
//! root_tag.insert_compound_tag_vec("servers", servers);
//!
//! let mut vec = Vec::new();
//! write_compound_tag(&mut vec, &root_tag).unwrap();
//! ```
use linked_hash_map::LinkedHashMap;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Error, Formatter};

pub mod decode;
pub mod encode;

/// Possible types of tags and they payload.
#[derive(Debug, Clone)]
pub enum Tag<'a> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(Cow<'a, str>),
    List(Vec<Tag<'a>>),
    Compound(CompoundTag<'a>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag<'_> {
    fn type_id(&self) -> u8 {
        match self {
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12,
        }
    }

    fn type_name(&self) -> &str {
        match self {
            Tag::Byte(_) => "TAG_Byte",
            Tag::Short(_) => "TAG_Short",
            Tag::Int(_) => "TAG_Int",
            Tag::Long(_) => "TAG_Long",
            Tag::Float(_) => "TAG_Float",
            Tag::Double(_) => "TAG_Double",
            Tag::ByteArray(_) => "TAG_Byte_Array",
            Tag::String(_) => "TAG_String",
            Tag::List(_) => "TAG_List",
            Tag::Compound(_) => "TAG_Compound",
            Tag::IntArray(_) => "TAG_Int_Array",
            Tag::LongArray(_) => "TAG_Long_Array",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompoundTag<'a> {
    pub name: Option<Cow<'a, str>>,
    tags: LinkedHashMap<Cow<'a, str>, Tag<'a>>,
}

/// Possible types of errors while trying to get value from compound tag.
#[derive(Debug)]
pub enum CompoundTagError {
    /// Tag with provided name not found.
    TagNotFound,
    /// Tag actual type not match expected.
    TagWrongType,
}

macro_rules! define_primitive_type (
    ($type: ident, $tag: ident, $getter_name: ident, $setter_name: ident) => (
        pub fn $setter_name(&mut self, name: &'a str, value: $type) {
            self.tags.insert(Cow::Borrowed(name), Tag::$tag(value));
        }

        pub fn $getter_name(&self, name: &str) -> Result<$type, CompoundTagError> {
            match self.tags.get(name) {
                Some(tag) => match tag {
                    Tag::$tag(value) => Ok(*value),
                    _ => Err(CompoundTagError::TagWrongType),
                },
                None => Err(CompoundTagError::TagNotFound),
            }
        }
   );
);

macro_rules! define_array_type (
    ($type: ident, $tag: ident, $getter_name: ident, $setter_name: ident) => (
        pub fn $setter_name(&mut self, name: &'a str, value: Vec<$type>) {
            self.tags.insert(Cow::Borrowed(name), Tag::$tag(value));
        }

        pub fn $getter_name(&self, name: &str) -> Result<&Vec<$type>, CompoundTagError> {
            match self.tags.get(name) {
                Some(tag) => match tag {
                    Tag::$tag(value) => Ok(value),
                    _ => Err(CompoundTagError::TagWrongType),
                },
                None => Err(CompoundTagError::TagNotFound),
            }
        }
   );
);

impl<'a> CompoundTag<'a> {
    pub fn new() -> Self {
        CompoundTag {
            name: None,
            tags: LinkedHashMap::new(),
        }
    }

    pub fn named(name: &'a str) -> Self {
        CompoundTag {
            name: Some(Cow::Borrowed(name)),
            tags: LinkedHashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.tags.contains_key(name)
    }

    define_primitive_type!(i8, Byte, get_i8, insert_i8);
    define_primitive_type!(i16, Short, get_i16, insert_i16);
    define_primitive_type!(i32, Int, get_i32, insert_i32);
    define_primitive_type!(i64, Long, get_i64, insert_i64);
    define_primitive_type!(f32, Float, get_f32, insert_f32);
    define_primitive_type!(f64, Double, get_f64, insert_f64);
    define_array_type!(i8, ByteArray, get_i8_vec, insert_i8_vec);
    define_array_type!(i32, IntArray, get_i32_vec, insert_i32_vec);
    define_array_type!(i64, LongArray, get_i64_vec, insert_i64_vec);

    pub fn insert_bool(&mut self, name: &'a str, value: bool) {
        if value {
            self.insert_i8(name, 1);
        } else {
            self.insert_i8(name, 0);
        }
    }

    pub fn get_bool(&self, name: &str) -> Result<bool, CompoundTagError> {
        Ok(self.get_i8(name)? == 1)
    }

    pub fn insert_str(&mut self, name: &'a str, value: &'a str) {
        self.tags
            .insert(Cow::Borrowed(name), Tag::String(Cow::Borrowed(value)));
    }

    pub fn get_str(&self, name: &str) -> Result<&str, CompoundTagError> {
        match self.tags.get(name) {
            Some(tag) => match tag {
                Tag::String(value) => Ok(value),
                _ => Err(CompoundTagError::TagWrongType),
            },
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    pub fn insert_compound_tag(&mut self, name: &'a str, value: CompoundTag<'a>) {
        self.tags.insert(Cow::Borrowed(name), Tag::Compound(value));
    }

    pub fn get_compound_tag(&self, name: &str) -> Result<&CompoundTag<'a>, CompoundTagError> {
        match self.tags.get(name) {
            Some(tag) => match tag {
                Tag::Compound(value) => Ok(value),
                _ => Err(CompoundTagError::TagWrongType),
            },
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    fn get_vec(&self, name: &str) -> Result<&Vec<Tag<'a>>, CompoundTagError> {
        match self.tags.get(name) {
            Some(tag) => match tag {
                Tag::List(value) => Ok(value),
                _ => Err(CompoundTagError::TagWrongType),
            },
            None => Err(CompoundTagError::TagNotFound),
        }
    }

    pub fn insert_str_vec(&mut self, name: &'a str, vec: Vec<&'a str>) {
        let mut tags = Vec::new();

        for value in vec {
            tags.push(Tag::String(Cow::Borrowed(value)));
        }

        self.tags.insert(Cow::Borrowed(name), Tag::List(tags));
    }

    pub fn get_str_vec<'b>(&'b self, name: &'b str) -> Result<Vec<&str>, CompoundTagError> {
        let tags = self.get_vec(name)?;
        let mut vec = Vec::new();

        for tag in tags {
            match tag {
                Tag::String(value) => vec.push(value.as_ref()),
                _ => return Err(CompoundTagError::TagWrongType),
            }
        }

        Ok(vec)
    }

    pub fn insert_compound_tag_vec(&mut self, name: &'a str, vec: Vec<CompoundTag<'a>>) {
        let mut tags = Vec::new();

        for value in vec {
            tags.push(Tag::Compound(value));
        }

        self.tags.insert(Cow::Borrowed(name), Tag::List(tags));
    }

    pub fn get_compound_tag_vec(
        &self,
        name: &str,
    ) -> Result<Vec<&CompoundTag<'a>>, CompoundTagError> {
        let tags = self.get_vec(name)?;
        let mut vec = Vec::new();

        for tag in tags {
            match tag {
                Tag::Compound(value) => vec.push(value),
                _ => return Err(CompoundTagError::TagWrongType),
            }
        }

        Ok(vec)
    }
}

impl Display for CompoundTag<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let name_ref = self.name.as_ref().map(|x| &**x);
        fmt_tag(f, name_ref, &Tag::Compound(self.clone()), 0)
    }
}

fn fmt_tag(f: &mut Formatter, name: Option<&str>, tag: &Tag, indent: usize) -> Result<(), Error> {
    fmt_indent(f, indent)?;

    let type_name = tag.type_name();

    match tag {
        Tag::Byte(value) => fmt_simple_tag(f, type_name, name, value)?,
        Tag::Short(value) => fmt_simple_tag(f, type_name, name, value)?,
        Tag::Int(value) => fmt_simple_tag(f, type_name, name, value)?,
        Tag::Long(value) => fmt_simple_tag(f, type_name, name, value)?,
        Tag::Float(value) => fmt_simple_tag(f, type_name, name, value)?,
        Tag::Double(value) => fmt_simple_tag(f, type_name, name, value)?,
        Tag::ByteArray(value) => fmt_array_tag(f, type_name, name, value)?,
        Tag::String(value) => fmt_simple_tag(f, type_name, name, value)?,
        Tag::List(value) => {
            let length = value.len();

            fmt_list_start(f, type_name, name, length)?;

            for tag in value {
                fmt_tag(f, None, tag, indent + 2)?;
            }

            if length > 0 {
                fmt_list_end(f, indent)?;
            }
        }
        Tag::Compound(value) => {
            let name_ref = name.as_ref().map(|x| &**x);
            let length = value.tags.len();

            fmt_list_start(f, type_name, name_ref, length)?;

            for (name, tag) in &value.tags {
                fmt_tag(f, Some(&name), tag, indent + 2)?;
            }

            if length > 0 {
                fmt_list_end(f, indent)?;
            }
        }
        Tag::IntArray(value) => fmt_array_tag(f, type_name, name, value)?,
        Tag::LongArray(value) => fmt_array_tag(f, type_name, name, value)?,
    };

    Ok(())
}

fn fmt_simple_tag<V: Display>(
    f: &mut Formatter,
    type_name: &str,
    name: Option<&str>,
    value: V,
) -> Result<(), Error> {
    writeln!(f, "{}('{}'): '{}'", type_name, fmt_str_opt(name), value)
}

fn fmt_array_tag<V: Debug>(
    f: &mut Formatter,
    type_name: &str,
    name: Option<&str>,
    value: V,
) -> Result<(), Error> {
    writeln!(f, "{}('{}'): '{:?}'", type_name, fmt_str_opt(name), value)
}

fn fmt_list_start(
    f: &mut Formatter,
    type_name: &str,
    name: Option<&str>,
    length: usize,
) -> Result<(), Error> {
    let fmt_name = fmt_str_opt(name);

    match length {
        0 => writeln!(f, "{}('{}'): 0 entries", type_name, fmt_name),
        1 => writeln!(f, "{}('{}'): 1 entry {{", type_name, fmt_name),
        _ => writeln!(f, "{}('{}'): {} entries {{", type_name, fmt_name, length),
    }
}

fn fmt_list_end(f: &mut Formatter, indent: usize) -> Result<(), Error> {
    fmt_indent(f, indent)?;
    writeln!(f, "}}")
}

fn fmt_indent(f: &mut Formatter, indent: usize) -> Result<(), Error> {
    write!(f, "{:indent$}", "", indent = indent)
}

fn fmt_str_opt(name: Option<&str>) -> &str {
    match name {
        Some(value) => value,
        None => "",
    }
}

#[test]
fn test_compound_tag_i8() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i8("i8", 1);

    assert_eq!(compound_tag.get_i8("i8").unwrap(), 1i8);
}

#[test]
fn test_compound_tag_bool() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_bool("bool", true);

    assert!(compound_tag.get_bool("bool").unwrap());
}

#[test]
fn test_compound_tag_i16() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i16("i16", 2);

    assert_eq!(compound_tag.get_i16("i16").unwrap(), 2i16);
}

#[test]
fn test_compound_tag_i32() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i32("i32", 3);

    assert_eq!(compound_tag.get_i32("i32").unwrap(), 3i32);
}

#[test]
fn test_compound_tag_i64() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i64("i64", 4);

    assert_eq!(compound_tag.get_i64("i64").unwrap(), 4i64);
}

#[test]
fn test_compound_tag_f32() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_f32("f32", 5.1);

    assert_eq!(compound_tag.get_f32("f32").unwrap(), 5.1f32);
}

#[test]
fn test_compound_tag_f64() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_f64("f64", 6.3322);

    assert_eq!(compound_tag.get_f64("f64").unwrap(), 6.3322f64);
}

#[test]
fn test_compound_tag_str() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_str("str", "hello world");

    assert_eq!(compound_tag.get_str("str").unwrap(), "hello world");
}

#[test]
fn test_compound_tag_nested_compound_tag() {
    let mut compound_tag = CompoundTag::new();
    let mut insert_nested_compound_tag = CompoundTag::named("nested");
    insert_nested_compound_tag.insert_i8("i8", 1);
    insert_nested_compound_tag.insert_str("str", "hello world");

    compound_tag.insert_compound_tag("nested_compound_tag", insert_nested_compound_tag);

    let get_nested_compound_tag = compound_tag
        .get_compound_tag("nested_compound_tag")
        .unwrap();

    assert_eq!(get_nested_compound_tag.get_i8("i8").unwrap(), 1i8);
    assert_eq!(
        get_nested_compound_tag.get_str("str").unwrap(),
        "hello world"
    );
}

#[test]
fn test_compound_tag_i8_vec() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i8_vec("i8_vec", vec![0, 1]);

    let i8_vec = compound_tag.get_i8_vec("i8_vec").unwrap();
    assert_eq!(i8_vec[0], 0);
    assert_eq!(i8_vec[1], 1);
}

#[test]
fn test_compound_tag_i32_vec() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i32_vec("i32_vec", vec![7, 8, 9]);

    let i32_vec = compound_tag.get_i32_vec("i32_vec").unwrap();

    assert_eq!(i32_vec[0], 7i32);
    assert_eq!(i32_vec[1], 8i32);
    assert_eq!(i32_vec[2], 9i32);
}

#[test]
fn test_compound_tag_i64_vec() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i64_vec("i64_vec", vec![10, 11, 12]);
    let i64_vec = compound_tag.get_i64_vec("i64_vec").unwrap();

    assert_eq!(i64_vec[0], 10i64);
    assert_eq!(i64_vec[1], 11i64);
    assert_eq!(i64_vec[2], 12i64);
}

#[test]
fn test_compound_tag_str_vec() {
    let mut compound_tag = CompoundTag::new();
    let insert_str_vec = vec!["a", "b", "c"];

    compound_tag.insert_str_vec("str_vec", insert_str_vec);

    let get_str_vec = compound_tag.get_str_vec("str_vec").unwrap();
    assert_eq!(get_str_vec[0], "a");
    assert_eq!(get_str_vec[1], "b");
    assert_eq!(get_str_vec[2], "c");
}

#[test]
fn test_compound_tag_nested_compound_tag_vec() {
    let mut compound_tag = CompoundTag::new();
    let mut insert_nested_compound_tag_1 = CompoundTag::new();
    let mut insert_nested_compound_tag_2 = CompoundTag::new();

    insert_nested_compound_tag_1.insert_str("str", "test");
    insert_nested_compound_tag_2.insert_i32("i32", 222333111);

    let insert_nested_compound_tag_vec =
        vec![insert_nested_compound_tag_1, insert_nested_compound_tag_2];

    compound_tag.insert_compound_tag_vec("nested_compound_tag_vec", insert_nested_compound_tag_vec);

    let get_nested_compound_tag_vec = compound_tag
        .get_compound_tag_vec("nested_compound_tag_vec")
        .unwrap();

    let get_nested_compound_tag_1 = get_nested_compound_tag_vec[0];
    let get_nested_compound_tag_2 = get_nested_compound_tag_vec[1];

    assert_eq!(get_nested_compound_tag_1.get_str("str").unwrap(), "test");
    assert_eq!(get_nested_compound_tag_2.get_i32("i32").unwrap(), 222333111);
}

#[test]
fn test_servers_display() {
    use crate::decode::read_compound_tag;
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../test/binary/servers.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

    assert_eq!(
        root_tag.to_string(),
        include_str!("../test/text/servers.txt")
    );
}

#[test]
fn test_hello_world_display() {
    use crate::decode::read_compound_tag;
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../test/binary/hello_world.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

    assert_eq!(
        root_tag.to_string(),
        include_str!("../test/text/hello_world.txt")
    );
}

#[test]
fn test_player_display() {
    use crate::decode::read_gzip_compound_tag;
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../test/binary/player.dat").to_vec());
    let root_tag = read_gzip_compound_tag(&mut cursor).unwrap();

    assert_eq!(
        root_tag.to_string(),
        include_str!("../test/text/player.txt")
    );
}

#[test]
fn test_level_display() {
    use crate::decode::read_gzip_compound_tag;
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../test/binary/level.dat").to_vec());
    let root_tag = read_gzip_compound_tag(&mut cursor).unwrap();

    assert_eq!(root_tag.to_string(), include_str!("../test/text/level.txt"));
}

#[test]
fn test_is_empty() {
    let mut compound_tag = CompoundTag::new();
    assert!(compound_tag.is_empty());

    compound_tag.insert_i32("test", 123);
    assert!(!compound_tag.is_empty());
}

#[test]
fn test_contains_key() {
    let mut compound_tag = CompoundTag::new();
    assert!(!compound_tag.contains_key("test"));

    compound_tag.insert_i32("test", 123);
    assert!(compound_tag.contains_key("test"));
    assert!(!compound_tag.contains_key("test2"));
}
