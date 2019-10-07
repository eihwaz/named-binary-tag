use std::collections::HashMap;

/// All possible types of tags.
enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(TagCompound),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
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

pub struct TagCompound {
    tags: HashMap<String, Tag>,
}

impl TagCompound {
    pub fn get_byte(&self, name: &str) -> Option<i8> {
        if let Some(Tag::Byte(value)) = self.tags.get(name) {
            return Some(*value);
        }

        None
    }

    pub fn get_boolean(&self, name: &str) -> Option<bool> {
        let byte = self.get_byte(name)?;
        Some(byte == 1)
    }

    pub fn get_string(&self, name: &str) -> Option<&str> {
        if let Some(Tag::String(value)) = self.tags.get(name) {
            return Some(&value);
        }

        None
    }

    pub fn get_compound(&self, name: &str) -> Option<&TagCompound> {
        if let Some(Tag::Compound(value)) = self.tags.get(name) {
            return Some(&value);
        }

        None
    }

    pub fn get_compound_list(&self, name: &str) -> Option<Vec<&TagCompound>> {
        if let Some(Tag::List(tags)) = self.tags.get(name) {
            let mut vec = Vec::new();

            for tag in tags {
                if let Tag::Compound(value) = tag {
                    vec.push(value)
                }
            }

            return Some(vec);
        }

        None
    }
}

pub fn read_from_vec(vec: Vec<u8>) -> TagCompound {
    TagCompound {
        tags: Default::default(),
    }
}

#[test]
fn test_servers_read() {
    let vec = include_bytes!("../test/servers.dat").to_vec();
    let root_tag = read_from_vec(vec);

    let servers = root_tag.get_compound_list("servers").unwrap();
    assert_eq!(servers.len(), 1);

    let server = servers.get(0).unwrap();
    let ip = server.get_string("ip").unwrap();
    let name = server.get_string("name").unwrap();
    let hide_address = server.get_boolean("hideAddress").unwrap();

    assert_eq!(ip, "localhost:25565");
    assert_eq!(name, "Minecraft Server");
    assert!(hide_address);
}
