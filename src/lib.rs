// Copyright (c) 2016 Jonathan Nilsson
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate persistent_array;
extern crate twox_hash;

use persistent_array::{Error, PersistentArray};
use std::borrow::Borrow;
use std::default::Default;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::path::Path;
use twox_hash::XxHash;

const OCCUPIED_MASK: u64 = 0x8000_0000_0000_0000;
const HASH_MASK: u64 = 0x7FFF_FFFF_FFFF_FFFF;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsertError {
    IsFull,
}

#[derive(Copy, Default)]
#[repr(C, packed)]
struct HashmapEntry<V: Copy> {
    // 1 bit occupied and 63 bit hash
    state: u64,
    value: V,
}

impl<V: Copy> Clone for HashmapEntry<V> {
    fn clone(&self) -> HashmapEntry<V> {
        *self
    }
}

pub struct PersistentHashmap<K: ?Sized, V: Copy> {
    phantom_type: PhantomData<K>,
    array: PersistentArray<HashmapEntry<V>>,
}

#[inline]
fn hash<K: ?Sized + Hash>(v: &K) -> u64 {
    let mut s = XxHash::with_seed(0);
    v.hash(&mut s);
    s.finish()
}

#[inline]
fn hash_equal(h1: u64, h2: u64) -> bool {
    h1 & HASH_MASK == h2 & HASH_MASK
}

#[inline]
fn state_is_occupeid(state: u64) -> bool {
    state & OCCUPIED_MASK == OCCUPIED_MASK
}

impl<K: ?Sized, V: Copy + Default> PersistentHashmap<K, V> {
    /// Creates a new persistent hashmap
    pub fn new<P>(path: P, size: u64) -> Result<PersistentHashmap<K, V>, Error>
    where
        P: AsRef<Path>,
    {
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
    where
        P: AsRef<Path>,
    {
        let array = match PersistentArray::open(&path) {
            Ok(array) => array,
            Err(err) => return Err(err),
        };

        Ok(PersistentHashmap {
            phantom_type: PhantomData,
            array: array,
        })
    }

    pub fn insert<Q: ?Sized>(&mut self, k: &Q, v: V) -> Result<Option<V>, InsertError>
    where
        K: Borrow<Q>,
        Q: Hash,
    {
        let (slot, hash) = self.get_slot_and_hash(k);

        let entry_slot = match self.find_entry_slot(slot, hash) {
            Some(slot) => slot,
            None => return Err(InsertError::IsFull),
        };

        let index: usize = entry_slot as usize;

        let entry = &mut self.array[index];

        if state_is_occupeid(entry.state) {
            let old = entry.value;
            entry.value = v;
            entry.state = hash;
            Ok(Some(old))
        } else {
            entry.value = v;
            entry.state = hash;
            Ok(None)
        }
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash,
    {
        let (slot, hash) = self.get_slot_and_hash(k);

        let entry_slot = match self.find_entry_slot(slot, hash) {
            Some(slot) => slot,
            None => return None,
        };

        let index: usize = entry_slot as usize;

        let entry = &self.array[index];

        if state_is_occupeid(entry.state) {
            Some(entry.value)
        } else {
            None
        }
    }

    #[inline]
    fn get_slot_and_hash<Q: ?Sized + Hash>(&self, k: &Q) -> (u64, u64) {
        let hash = hash(k) | OCCUPIED_MASK;
        (hash % self.array.len() as u64, hash)
    }

    #[inline]
    fn find_entry_slot(&self, start_slot: u64, hash: u64) -> Option<u64> {
        let array_slice: &[HashmapEntry<V>] = &*self.array;
        let size = array_slice.len() as u64;
        let mut slot_counter = start_slot;
        let mut index: usize = slot_counter as usize;

        while !hash_equal(array_slice[index].state, hash)
            && state_is_occupeid(array_slice[index].state)
        {
            slot_counter = (slot_counter + 1) % size;
            index = slot_counter as usize;

            if slot_counter == start_slot {
                return None;
            }
        }

        Some(slot_counter)
    }
}
