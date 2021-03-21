named-binary-tag
============
[![crates.io](https://img.shields.io/crates/v/named-binary-tag.svg)](https://crates.io/crates/named-binary-tag)
[![Build Status](https://travis-ci.com/eihwaz/named-binary-tag.svg?branch=master)](https://travis-ci.com/eihwaz/named-binary-tag)
[![codecov](https://codecov.io/gh/eihwaz/named-binary-tag/branch/master/graph/badge.svg)](https://codecov.io/gh/eihwaz/named-binary-tag)

NBT (Named Binary Tag) is a tag based binary format designed to carry large amounts of binary data with smaller amounts of additional data.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
named-binary-tag = "0.6"
```

## Example

#### Read

```rust
use nbt::decode::read_compound_tag;
use std::io::Cursor;

let mut cursor = Cursor::new(include_bytes!("../test/binary/servers.dat").to_vec());
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
```

#### Write

```rust
use nbt::encode::write_compound_tag;
use nbt::CompoundTag;

let mut server = CompoundTag::new();

server.insert_str("ip", "localhost:25565");
server.insert_str("name", "Minecraft Server");
server.insert_bool("hideAddress", true);

let mut servers = Vec::new();
servers.push(server);

let mut root_tag = CompoundTag::new();
root_tag.insert_compound_tag_vec("servers", servers);

let mut vec = Vec::new();
write_compound_tag(&mut vec, &root_tag).unwrap();
```