#![feature(reflect_marker)]

extern crate persistent_array;

use std::marker::Reflect;
use std::path::Path;

use persistent_array::{PersistentArray, Error};

pub struct PersistentHashmap<T> where T: Copy + Default + Reflect + 'static {
    array: PersistentArray<T>,
}

impl<T> PersistentHashmap<T> where T: Copy + Default + Reflect + 'static {

    /// Creates a new persistent hashmap
    pub fn new<P>(path: P, size: u64) -> Result<PersistentHashmap<T>, Error>
            where P: AsRef<Path> {
        let array = match PersistentArray::new(&path, size) {
            Ok(array) => array,
            Err(err) => return Err(err),
        };

        Ok(PersistentHashmap {
            array: array,
        })
    }

    /// Opens an existing persistent hashmap
    pub fn open<P>(path: P) -> Result<PersistentHashmap<T>, Error>
            where P: AsRef<Path> {

        let array = match PersistentArray::open(&path) {
            Ok(array) => array,
            Err(err) => return Err(err),
        };

        Ok(PersistentHashmap {
            array: array,
        })
    }
}