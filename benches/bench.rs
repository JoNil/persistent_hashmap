#![feature(test)]

extern crate persistent_hashmap;
extern crate test;

use persistent_hashmap::PersistentHashmap;
use std::mem::size_of;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    
    let capacity = 10*1024;
    let size = 1*1024;

    {
        let mut db = PersistentHashmap::<u64, u64>::new("bench.db", capacity).unwrap();

        for i in 0..size {
            assert_eq!(db.insert(i, i), Ok(None));    
        }
    }

    let db = PersistentHashmap::<u64, u64>::open("bench.db").unwrap();

    b.bytes = size * size_of::<u64>() as u64;
    b.iter(|| {
        for i in 0..size {
            assert_eq!(db.get(i), Some(i));    
        }
    });
}
