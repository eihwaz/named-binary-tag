use std::collections::HashMap;
use std::io::{Cursor, Read, Write};

#[derive(Debug)]
pub enum TagError {
    NotFound,
    WrongType,
}

/// Possible types of tags.
enum Tag<'a> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(&'a str),
    List(Vec<Tag<'a>>),
    Compound(CompoundTag<'a>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl<'a> Tag<'a> {
    /// Returns tag id.
    fn id(&self) -> u8 {
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
}

pub struct CompoundTag<'a> {
    tags: HashMap<&'a str, Tag<'a>>,
}

macro_rules! define_primitive_type (
    ($type: ident, $tag: ident, $getter_name: ident, $setter_name: ident) => (
        pub fn $setter_name(&mut self, name: &'a str, value: $type) {
            self.tags.insert(name, Tag::$tag(value));
        }

        pub fn $getter_name(&self, name: &str) -> Result<$type, TagError> {
            match self.tags.get(name) {
                Some(tag) => match tag {
                    Tag::$tag(value) => Ok(*value),
                    _ => Err(TagError::WrongType),
                },
                None => Err(TagError::NotFound),
            }
        }
   );
);

macro_rules! define_array_type (
    ($type: ident, $tag: ident, $getter_name: ident, $setter_name: ident) => (
        pub fn $setter_name(&mut self, name: &'a str, value: Vec<$type>) {
            self.tags.insert(name, Tag::$tag(value));
        }

        pub fn $getter_name(&self, name: &str) -> Result<&Vec<$type>, TagError> {
            match self.tags.get(name) {
                Some(tag) => match tag {
                    Tag::$tag(value) => Ok(value),
                    _ => Err(TagError::WrongType),
                },
                None => Err(TagError::NotFound),
            }
        }
   );
);

impl<'a> CompoundTag<'a> {
    pub fn new() -> Self {
        CompoundTag {
            tags: HashMap::new(),
        }
    }

    define_primitive_type!(i8, Byte, get_i8, set_i8);
    define_primitive_type!(i16, Short, get_i16, set_i16);
    define_primitive_type!(i32, Int, get_i32, set_i32);
    define_primitive_type!(i64, Long, get_i64, set_i64);
    define_primitive_type!(f32, Float, get_f32, set_f32);
    define_primitive_type!(f64, Double, get_f64, set_f64);
    define_array_type!(i8, ByteArray, get_i8_vec, set_i8_vec);
    define_array_type!(i32, IntArray, get_i32_vec, set_i32_vec);
    define_array_type!(i64, LongArray, get_i64_vec, set_i64_vec);

    pub fn set_bool(&mut self, name: &'a str, value: bool) {
        if value {
            self.set_i8(name, 1);
        } else {
            self.set_i8(name, 0);
        }
    }

    pub fn get_bool(&self, name: &str) -> Result<bool, TagError> {
        Ok(self.get_i8(name)? == 1)
    }

    pub fn set_str(&mut self, name: &'a str, value: &'a str) {
        self.tags.insert(name, Tag::String(value));
    }

    pub fn get_str(&self, name: &str) -> Result<&'a str, TagError> {
        match self.tags.get(name) {
            Some(tag) => match tag {
                Tag::String(value) => Ok(value),
                _ => Err(TagError::WrongType),
            },
            None => Err(TagError::NotFound),
        }
    }

    pub fn set_compound_tag(&mut self, name: &'a str, value: CompoundTag<'a>) {
        self.tags.insert(name, Tag::Compound(value));
    }

    pub fn get_compound_tag(&self, name: &str) -> Result<&'a CompoundTag, TagError> {
        match self.tags.get(name) {
            Some(tag) => match tag {
                Tag::Compound(value) => Ok(value),
                _ => Err(TagError::WrongType),
            },
            None => Err(TagError::NotFound),
        }
    }

    fn get_vec(&self, name: &str) -> Result<&Vec<Tag<'a>>, TagError> {
        match self.tags.get(name) {
            Some(tag) => match tag {
                Tag::List(value) => Ok(value),
                _ => Err(TagError::WrongType),
            },
            None => Err(TagError::NotFound),
        }
    }

    pub fn set_str_vec(&mut self, name: &'a str, vec: Vec<&'a str>) {
        let mut tags = Vec::new();

        for value in vec {
            tags.push(Tag::String(value));
        }

        self.tags.insert(name, Tag::List(tags));
    }

    pub fn get_str_vec(&self, name: &str) -> Result<Vec<&'a str>, TagError> {
        let tags = self.get_vec(name)?;
        let mut vec = Vec::new();

        for tag in tags {
            match tag {
                Tag::String(value) => vec.push(*value),
                _ => return Err(TagError::WrongType),
            }
        }

        Ok(vec)
    }

    pub fn set_compound_tag_vec(&mut self, name: &'a str, vec: Vec<CompoundTag<'a>>) {
        let mut tags = Vec::new();

        for value in vec {
            tags.push(Tag::Compound(value));
        }

        self.tags.insert(name, Tag::List(tags));
    }

    pub fn get_compound_tag_vec(&self, name: &str) -> Result<Vec<&'a CompoundTag>, TagError> {
        let tags = self.get_vec(name)?;
        let mut vec = Vec::new();

        for tag in tags {
            match tag {
                Tag::Compound(value) => vec.push(value),
                _ => return Err(TagError::WrongType),
            }
        }

        Ok(vec)
    }
}

pub fn read_compound_tag<'a, R: Read>(reader: &mut R) -> Result<CompoundTag<'a>, TagError> {
    Ok(CompoundTag::new())
}

pub fn write_compound_tag<W: Write>(
    writer: &mut W,
    compound_tag: CompoundTag,
) -> Result<(), TagError> {
    Ok(())
}

#[test]
fn test_compound_tag_i8() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_i8("i8", 1);

    assert_eq!(compound_tag.get_i8("i8").unwrap(), 1i8);
}

#[test]
fn test_compound_tag_bool() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_bool("bool", true);

    assert!(compound_tag.get_bool("bool").unwrap());
}

#[test]
fn test_compound_tag_i16() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_i16("i16", 2);

    assert_eq!(compound_tag.get_i16("i16").unwrap(), 2i16);
}

#[test]
fn test_compound_tag_i32() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_i32("i32", 3);

    assert_eq!(compound_tag.get_i32("i32").unwrap(), 3i32);
}

#[test]
fn test_compound_tag_i64() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_i64("i64", 4);

    assert_eq!(compound_tag.get_i64("i64").unwrap(), 4i64);
}

#[test]
fn test_compound_tag_f32() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_f32("f32", 5.1);

    assert_eq!(compound_tag.get_f32("f32").unwrap(), 5.1f32);
}

#[test]
fn test_compound_tag_f64() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_f64("f64", 6.3322);

    assert_eq!(compound_tag.get_f64("f64").unwrap(), 6.3322f64);
}

#[test]
fn test_compound_tag_str() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_str("str", "hello world");

    assert_eq!(compound_tag.get_str("str").unwrap(), "hello world");
}

#[test]
fn test_compound_tag_nested_compound_tag() {
    let mut compound_tag = CompoundTag::new();
    let mut set_nested_compound_tag = CompoundTag::new();
    set_nested_compound_tag.set_i8("i8", 1);
    set_nested_compound_tag.set_str("str", "hello world");

    compound_tag.set_compound_tag("nested_compound_tag", set_nested_compound_tag);

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
    compound_tag.set_i8_vec("i8_vec", vec![0, 1]);

    let i8_vec = compound_tag.get_i8_vec("i8_vec").unwrap();
    assert_eq!(i8_vec[0], 0);
    assert_eq!(i8_vec[1], 1);
}

#[test]
fn test_compound_tag_i32_vec() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_i32_vec("i32_vec", vec![7, 8, 9]);

    let i32_vec = compound_tag.get_i32_vec("i32_vec").unwrap();

    assert_eq!(i32_vec[0], 7i32);
    assert_eq!(i32_vec[1], 8i32);
    assert_eq!(i32_vec[2], 9i32);
}

#[test]
fn test_compound_tag_i64_vec() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.set_i64_vec("i64_vec", vec![10, 11, 12]);
    let i64_vec = compound_tag.get_i64_vec("i64_vec").unwrap();

    assert_eq!(i64_vec[0], 10i64);
    assert_eq!(i64_vec[1], 11i64);
    assert_eq!(i64_vec[2], 12i64);
}

#[test]
fn test_compound_tag_str_vec() {
    let mut compound_tag = CompoundTag::new();
    let set_str_vec = vec!["a", "b", "c"];

    compound_tag.set_str_vec("str_vec", set_str_vec);

    let get_str_vec = compound_tag.get_str_vec("str_vec").unwrap();
    assert_eq!(get_str_vec[0], "a");
    assert_eq!(get_str_vec[1], "b");
    assert_eq!(get_str_vec[2], "c");
}

#[test]
fn test_compound_tag_nested_compound_tag_vec() {
    let mut compound_tag = CompoundTag::new();
    let mut set_nested_compound_tag_1 = CompoundTag::new();
    let mut set_nested_compound_tag_2 = CompoundTag::new();

    set_nested_compound_tag_1.set_str("str", "test");
    set_nested_compound_tag_2.set_i32("i32", 222333111);

    let set_nested_compound_tag_vec = vec![set_nested_compound_tag_1, set_nested_compound_tag_2];

    compound_tag.set_compound_tag_vec("nested_compound_tag_vec", set_nested_compound_tag_vec);

    let get_nested_compound_tag_vec = compound_tag
        .get_compound_tag_vec("nested_compound_tag_vec")
        .unwrap();

    let get_nested_compound_tag_1 = get_nested_compound_tag_vec[0];
    let get_nested_compound_tag_2 = get_nested_compound_tag_vec[1];

    assert_eq!(get_nested_compound_tag_1.get_str("str").unwrap(), "test");
    assert_eq!(get_nested_compound_tag_2.get_i32("i32").unwrap(), 222333111);
}

#[test]
fn test_servers_read() {
    let cursor = Cursor::new(include_bytes!("../test/servers.dat").to_vec());
    let root_tag = read_compound_tag(cursor);

    let servers = root_tag.get_compound_tag_vec("servers").unwrap();
    assert_eq!(servers.len(), 1);

    let server = servers[0];
    let ip = server.get_str("ip").unwrap();
    let name = server.get_str("name").unwrap();
    let hide_address = server.get_bool("hideAddress").unwrap();

    assert_eq!(ip, "localhost:25565");
    assert_eq!(name, "Minecraft Server");
    assert!(hide_address);
}

#[test]
fn test_servers_write() {
    let mut server = CompoundTag::new();

    server.set_str("ip", "localhost:25565");
    server.set_str("name", "Minecraft Server");
    server.set_bool("hideAddress", true);

    let mut servers = Vec::new();
    servers.push(server);

    let mut root_tag = CompoundTag::new();
    root_tag.set_compound_tag_vec("servers", servers);

    let vec = Vec::new();
    write_compound_tag(&vec, root_tag);

    assert_eq!(vec, include_bytes!("../test/servers.dat").to_vec());
}
