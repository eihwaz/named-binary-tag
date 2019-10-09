use std::collections::HashMap;

/// All possible types of tags.
pub enum Tag<'a> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(&'a str),
    List(Vec<Tag<'a>>),
    Compound(HashMap<String, Tag<'a>>),
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

    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Tag::Byte(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        Some(self.as_i8()? == 1)
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Tag::Short(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Tag::Int(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Tag::Long(value) => Some(*value),
            _ => None,
        }
    }

    fn as_f32(&self) -> Option<f32> {
        match self {
            Tag::Float(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Tag::Double(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i8_vec(&self) -> Option<&Vec<i8>> {
        match self {
            Tag::ByteArray(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            Tag::String(value) => Some(value),
            _ => None,
        }
    }

    fn as_vec(&self) -> Option<&Vec<Tag>> {
        match self {
            Tag::List(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_str_vec(&'a self) -> Option<Vec<&'a str>> {
        let vec = self
            .as_vec()?
            .iter()
            .filter_map(|tag| tag.as_str())
            .collect();

        return Some(vec);
    }

    pub fn as_map_vec(&'a self) -> Option<Vec<&HashMap<String, Tag<'a>>>> {
        let vec = self
            .as_vec()?
            .iter()
            .filter_map(|tag| tag.as_map())
            .collect();

        return Some(vec);
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Tag<'a>>> {
        match self {
            Tag::Compound(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_i32_vec(&self) -> Option<&Vec<i32>> {
        match self {
            Tag::IntArray(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_i64_vec(&self) -> Option<&Vec<i64>> {
        match self {
            Tag::LongArray(value) => Some(value),
            _ => None,
        }
    }
}

pub fn read_from_vec<'a>(_vec: Vec<u8>) -> Tag<'a> {
    Tag::Compound(HashMap::new())
}

#[test]
fn test_servers_read() {
    let vec = include_bytes!("../test/servers.dat").to_vec();
    let root_tag = read_from_vec(vec);
    let servers = root_tag
        .as_map()
        .unwrap()
        .get("servers")
        .unwrap()
        .as_map_vec()
        .unwrap();

    assert_eq!(servers.len(), 1);

    let server = servers.get(0).unwrap();
    let ip = server.get("ip").unwrap().as_str().unwrap();
    let name = server.get("name").unwrap().as_str().unwrap();
    let hide_address = server.get("hideAddress").unwrap().as_bool().unwrap();

    assert_eq!(ip, "localhost:25565");
    assert_eq!(name, "Minecraft Server");
    assert!(hide_address);
}
