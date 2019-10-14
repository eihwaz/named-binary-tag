use crate::{CompoundTag, Tag};
use byteorder::{BigEndian, ReadBytesExt};
use linked_hash_map::LinkedHashMap;
use std::io;
use std::io::Read;

/// Possible types of errors while decoding tag.
#[derive(Debug)]
pub enum TagDecodeError {
    /// Root of tag must be compound tag.
    RootMustBeCompoundTag {
        /// Actual tag.
        actual_tag: Tag,
    },
    /// Tag type not recognized.
    UnknownTagType {
        /// Tag type id which is not recognized.
        tag_type_id: u8,
    },
    /// I/O Error which happened while were decoding.
    IOError { io_error: io::Error },
}

impl From<io::Error> for TagDecodeError {
    fn from(io_error: io::Error) -> Self {
        TagDecodeError::IOError { io_error }
    }
}

pub fn read_compound_tag<'a, R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
    let tag_id = reader.read_u8()?;
    let (_, tag) = read_tag(tag_id, reader, true)?;

    match tag {
        Tag::Compound(value) => Ok(value),
        actual_tag => Err(TagDecodeError::RootMustBeCompoundTag { actual_tag }),
    }
}

fn read_tag<R: Read>(
    id: u8,
    reader: &mut R,
    read_name: bool,
) -> Result<(String, Tag), TagDecodeError> {
    if id == 0 {
        return Ok(("".to_owned(), Tag::End));
    }

    let name = if read_name {
        read_string(reader)?
    } else {
        String::from("")
    };

    match id {
        1 => {
            let value = reader.read_i8()?;
            let tag = Tag::Byte(value);

            return Ok((name, tag));
        }
        2 => {
            let value = reader.read_i16::<BigEndian>()?;
            let tag = Tag::Short(value);

            return Ok((name, tag));
        }
        3 => {
            let value = reader.read_i32::<BigEndian>()?;
            let tag = Tag::Int(value);

            return Ok((name, tag));
        }
        4 => {
            let value = reader.read_i64::<BigEndian>()?;
            let tag = Tag::Long(value);

            return Ok((name, tag));
        }
        5 => {
            let value = reader.read_f32::<BigEndian>()?;
            let tag = Tag::Float(value);

            return Ok((name, tag));
        }
        6 => {
            let value = reader.read_f64::<BigEndian>()?;
            let tag = Tag::Double(value);

            return Ok((name, tag));
        }
        7 => {
            let length = reader.read_u32::<BigEndian>()?;
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i8()?);
            }

            let tag = Tag::ByteArray(value);

            return Ok((name, tag));
        }
        8 => {
            let value = read_string(reader)?;
            let tag = Tag::String(value);

            return Ok((name, tag));
        }
        9 => {
            let list_tags_id = reader.read_u8()?;
            let length = reader.read_u32::<BigEndian>()?;
            let mut value = Vec::new();

            for _ in 0..length {
                let (_, tag) = read_tag(list_tags_id, reader, false)?;
                value.push(tag);
            }

            let tag = Tag::List(value);

            return Ok((name, tag));
        }
        10 => {
            let mut tags = LinkedHashMap::new();

            loop {
                let tag_id = reader.read_u8()?;
                let (name, tag) = read_tag(tag_id, reader, true)?;

                match tag {
                    Tag::End => break,
                    _ => {
                        tags.insert(name, tag);
                    }
                }
            }

            let compound_tag = CompoundTag { tags };
            let tag = Tag::Compound(compound_tag);

            return Ok((name, tag));
        }
        11 => {
            let length = reader.read_u32::<BigEndian>()?;
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i32::<BigEndian>()?);
            }

            let tag = Tag::IntArray(value);

            return Ok((name, tag));
        }
        12 => {
            let length = reader.read_u32::<BigEndian>()?;
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i64::<BigEndian>()?);
            }

            let tag = Tag::LongArray(value);

            return Ok((name, tag));
        }
        tag_type_id => return Err(TagDecodeError::UnknownTagType { tag_type_id }),
    }
}

fn read_string<R: Read>(reader: &mut R) -> Result<String, TagDecodeError> {
    let length = reader.read_u16::<BigEndian>()?;
    let mut buf = vec![0; length as usize];
    reader.read_exact(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).into_owned())
}

#[test]
fn test_servers_read() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../test/servers.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

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
