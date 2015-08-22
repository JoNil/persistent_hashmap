#![feature(test)]

extern crate persistent_hashmap;
extern crate test;

use persistent_hashmap::PersistentHashmap;

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| {
        PersistentHashmap::<u64>::new("bench.db", 1024).unwrap();
    });
}
