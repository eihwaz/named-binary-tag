use crate::{CompoundTag, Tag, TagError};
use byteorder::{BigEndian, ReadBytesExt};
use linked_hash_map::LinkedHashMap;
use std::io::Read;

pub fn read_compound_tag<'a, R: Read>(reader: &mut R) -> Result<CompoundTag, TagError<'a>> {
    let tag_id = reader.read_u8().unwrap();
    let (_, tag) = read_tag(tag_id, reader, true)?;

    match tag {
        Tag::Compound(value) => Ok(value),
        _ => Err(TagError::ExpectedCompoundTag {
            unexpected_tag_id: tag.id(),
        }),
    }
}

fn read_tag<'a, R: Read>(
    id: u8,
    reader: &mut R,
    read_name: bool,
) -> Result<(String, Tag), TagError<'a>> {
    if id == 0 {
        return Ok(("".to_owned(), Tag::End));
    }

    let name = if read_name {
        read_string(reader)
    } else {
        String::from("")
    };

    match id {
        1 => {
            let value = reader.read_i8().unwrap();
            let tag = Tag::Byte(value);

            return Ok((name, tag));
        }
        2 => {
            let value = reader.read_i16::<BigEndian>().unwrap();
            let tag = Tag::Short(value);

            return Ok((name, tag));
        }
        3 => {
            let value = reader.read_i32::<BigEndian>().unwrap();
            let tag = Tag::Int(value);

            return Ok((name, tag));
        }
        4 => {
            let value = reader.read_i64::<BigEndian>().unwrap();
            let tag = Tag::Long(value);

            return Ok((name, tag));
        }
        5 => {
            let value = reader.read_f32::<BigEndian>().unwrap();
            let tag = Tag::Float(value);

            return Ok((name, tag));
        }
        6 => {
            let value = reader.read_f64::<BigEndian>().unwrap();
            let tag = Tag::Double(value);

            return Ok((name, tag));
        }
        7 => {
            let length = reader.read_u32::<BigEndian>().unwrap();
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i8().unwrap());
            }

            let tag = Tag::ByteArray(value);

            return Ok((name, tag));
        }
        8 => {
            let value = read_string(reader);
            let tag = Tag::String(value);

            return Ok((name, tag));
        }
        9 => {
            let list_tags_id = reader.read_u8().unwrap();
            let length = reader.read_u32::<BigEndian>().unwrap();
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
                let tag_id = reader.read_u8().unwrap();
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
            let length = reader.read_u32::<BigEndian>().unwrap();
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i32::<BigEndian>().unwrap());
            }

            let tag = Tag::IntArray(value);

            return Ok((name, tag));
        }
        12 => {
            let length = reader.read_u32::<BigEndian>().unwrap();
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i64::<BigEndian>().unwrap());
            }

            let tag = Tag::LongArray(value);

            return Ok((name, tag));
        }
        type_id => return Err(TagError::UnknownType { type_id }),
    }
}

fn read_string<R: Read>(reader: &mut R) -> String {
    let length = reader.read_u16::<BigEndian>().unwrap();
    let mut buf = vec![0; length as usize];
    reader.read_exact(&mut buf).unwrap();

    String::from_utf8_lossy(&buf).into_owned()
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
