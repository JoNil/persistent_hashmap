#![feature(reflect_marker)]

extern crate persistent_array;

use std::hash::{Hash, Hasher, SipHasher};
use std::marker::{PhantomData, Reflect};
use std::path::Path;

use persistent_array::{PersistentArray, Error};

pub trait KeyTypeBounds: Hash {}
impl<T: Hash> KeyTypeBounds for T {}

pub trait ValueTypeBounds: Copy + Default + Reflect + 'static {}
impl<T: Copy + Default + Reflect + 'static> ValueTypeBounds for T {}

#[derive(Copy, Clone, Default)]
pub struct HashmapEntry<V> {
    hash: u64,
    is_occupied: bool,
    value: V,
}

pub struct PersistentHashmap<K: ?Sized + KeyTypeBounds, V: ValueTypeBounds> {
    phantom_type: PhantomData<K>,
    array: PersistentArray<HashmapEntry<V>>,
}

fn hash<T: Hash>(v: &T) -> u64 {
    let mut s = SipHasher::new();
    v.hash(&mut s);
    s.finish()
}

impl<K: ?Sized + KeyTypeBounds, V: ValueTypeBounds> PersistentHashmap<K, V> {

    /// Creates a new persistent hashmap
    pub fn new<P>(path: P, size: u64) -> Result<PersistentHashmap<K, V>, Error>
            where P: AsRef<Path> {
        let array = match PersistentArray::new(&path, size) {
            Ok(array) => array,
            Err(err) => return Err(err),
        };

        Ok(PersistentHashmap {
            phantom_type: PhantomData,
            array: array,
        })
    }

    /// Opens an existing persistent hashmap
    pub fn open<P>(path: P) -> Result<PersistentHashmap<K, V>, Error>
            where P: AsRef<Path> {

        let array = match PersistentArray::open(&path) {
            Ok(array) => array,
            Err(err) => return Err(err),
        };

        Ok(PersistentHashmap {
            phantom_type: PhantomData,
            array: array,
        })
    }
}