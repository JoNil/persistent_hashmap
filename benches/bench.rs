// Copyright (c) 2016 Jonathan Nilsson
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![feature(test)]

extern crate persistent_hashmap;
extern crate test;

use persistent_hashmap::PersistentHashmap;
use std::mem::size_of;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    let capacity = 10 * 1024;
    let size = 1024;

    {
        let mut db = PersistentHashmap::<u64, u64>::new("bench.db", capacity).unwrap();

        for i in 0..size {
            assert_eq!(db.insert(&i, i), Ok(None));
        }
    }

    let db = PersistentHashmap::<u64, u64>::open("bench.db").unwrap();

    b.bytes = size * size_of::<u64>() as u64;
    b.iter(|| {
        for i in 0..size {
            assert_eq!(db.get(&i), Some(i));
        }
    });
}
