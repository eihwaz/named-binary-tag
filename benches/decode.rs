use criterion::{criterion_group, criterion_main, Criterion};
use nbt::decode::read_compound_tag;
use std::io::Cursor;

fn hello_world_read(c: &mut Criterion) {
    let data = include_bytes!("../test/binary/hello_world.dat").to_vec();

    c.bench_function("Bench hello world read", |b| {
        b.iter(|| {
            let mut cursor = Cursor::new(&data);
            read_compound_tag(&mut cursor).expect("Failed to read tag data");
        });
    });
}

fn servers_list_read(c: &mut Criterion) {
    let data = include_bytes!("../test/binary/servers.dat").to_vec();

    c.bench_function("Bench servers list read", |b| {
        b.iter(|| {
            let mut cursor = Cursor::new(&data);
            read_compound_tag(&mut cursor).expect("Failed to read tag data");
        });
    });
}

fn big_test_read(c: &mut Criterion) {
    let data = include_bytes!("../test/binary/bigtest.dat").to_vec();

    c.bench_function("Bench big test read", |b| {
        b.iter(|| {
            let mut cursor = Cursor::new(&data);
            read_compound_tag(&mut cursor).expect("Failed to read tag data");
        });
    });
}

criterion_group!(benches, hello_world_read, servers_list_read, big_test_read);
criterion_main!(benches);
