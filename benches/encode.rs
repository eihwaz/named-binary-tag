use criterion::{criterion_group, criterion_main, Criterion};
use nbt::encode::write_compound_tag;
use nbt::CompoundTag;

fn hello_world_write(c: &mut Criterion) {
    let mut hello_world = CompoundTag::named("hello world");
    hello_world.insert_str("name", "Bananrama");

    c.bench_function("Bench hello world write", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            write_compound_tag(&mut vec, hello_world.clone()).expect("Failed to write tag data");
        });
    });
}

fn servers_list_write(c: &mut Criterion) {
    let mut server = CompoundTag::new();

    server.insert_str("ip", "localhost:25565");
    server.insert_str("name", "Minecraft Server");
    server.insert_bool("hideAddress", true);

    let mut servers = Vec::new();
    servers.push(server);

    let mut root_tag = CompoundTag::new();
    root_tag.insert_compound_tag_vec("servers", servers);

    c.bench_function("Bench servers list write", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            write_compound_tag(&mut vec, root_tag.clone()).expect("Failed to write tag data");
        });
    });
}

criterion_group!(benches, hello_world_write, servers_list_write);
criterion_main!(benches);
