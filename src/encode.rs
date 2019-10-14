use crate::{CompoundTag, Tag, TagError};
use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

pub fn write_compound_tag<'a, W: Write>(
    writer: &mut W,
    compound_tag: CompoundTag,
) -> Result<(), TagError<'a>> {
    write_tag(writer, String::from(""), Tag::Compound(compound_tag), true)
}

fn write_tag<'a, W: Write>(
    writer: &mut W,
    name: String,
    tag: Tag,
    write_header: bool,
) -> Result<(), TagError<'a>> {
    if write_header {
        writer.write_u8(tag.id()).unwrap();
        write_string(writer, name);
    }

    match tag {
        Tag::Byte(value) => writer.write_i8(value).unwrap(),
        Tag::Short(value) => writer.write_i16::<BigEndian>(value).unwrap(),
        Tag::Int(value) => writer.write_i32::<BigEndian>(value).unwrap(),
        Tag::Long(value) => writer.write_i64::<BigEndian>(value).unwrap(),
        Tag::Float(value) => writer.write_f32::<BigEndian>(value).unwrap(),
        Tag::Double(value) => writer.write_f64::<BigEndian>(value).unwrap(),
        Tag::ByteArray(value) => {
            writer.write_u32::<BigEndian>(value.len() as u32).unwrap();

            for v in value {
                writer.write_i8(v).unwrap();
            }
        }
        Tag::String(value) => write_string(writer, value),
        Tag::List(value) => {
            if value.len() > 0 {
                writer.write_u8(value[0].id()).unwrap()
            } else {
                writer.write_u8(Tag::End.id()).unwrap()
            }

            writer.write_u32::<BigEndian>(value.len() as u32).unwrap();

            for tag in value {
                write_tag(writer, String::from(""), tag, false)?;
            }
        }
        Tag::Compound(value) => {
            for (name, tag) in value.tags {
                write_tag(writer, name, tag, true)?;
            }

            writer.write_u8(Tag::End.id()).unwrap();
        }
        Tag::IntArray(value) => {
            writer.write_u32::<BigEndian>(value.len() as u32).unwrap();

            for v in value {
                writer.write_i32::<BigEndian>(v).unwrap();
            }
        }
        Tag::LongArray(value) => {
            writer.write_u32::<BigEndian>(value.len() as u32).unwrap();

            for v in value {
                writer.write_i64::<BigEndian>(v).unwrap();
            }
        }
        _ => return Err(TagError::UnknownType { type_id: tag.id() }),
    }

    Ok(())
}

fn write_string<W: Write>(writer: &mut W, value: String) {
    writer.write_u16::<BigEndian>(value.len() as u16).unwrap();
    writer.write(value.as_bytes()).unwrap();
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

    let mut vec = Vec::new();
    write_compound_tag(&mut vec, root_tag).unwrap();

    assert_eq!(vec, include_bytes!("../test/servers.dat").to_vec());
}
