#![feature(reflect_marker)]

extern crate persistent_array;

use std::default::Default;
use std::hash::{Hash, Hasher, SipHasher};
use std::marker::{PhantomData, Reflect};
use std::path::Path;

use persistent_array::{Error, PersistentArray};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsertError {
    IsFull,
}

pub trait KeyTypeBounds: Hash {}
impl<T: Hash> KeyTypeBounds for T {}

pub trait ValueTypeBounds: Copy + Default + Reflect + 'static {}
impl<T: Copy + Default + Reflect + 'static> ValueTypeBounds for T {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum EntryState {
    Empty,
    Occupied,
}

impl Default for EntryState {
    fn default() -> EntryState { EntryState::Empty }
}

#[derive(Clone, Copy, Default)]
#[repr(C, packed)]
struct HashmapEntry<V> {
    hash: u64,
    state: EntryState,
    value: V,
}

pub struct PersistentHashmap<K: KeyTypeBounds, V: ValueTypeBounds> {
    phantom_type: PhantomData<K>,
    array: PersistentArray<HashmapEntry<V>>,
}

fn hash<K: Hash>(v: K) -> u64 {
    let mut s = SipHasher::new();
    v.hash(&mut s);
    s.finish()
}

impl<K: KeyTypeBounds, V: ValueTypeBounds> PersistentHashmap<K, V> {

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

    pub fn insert(&mut self, k: K, v: V) -> Result<Option<V>, InsertError> {
        let (slot, hash) = self.get_slot_and_hash(k);

        let size = self.array.len() as u64;
        let mut slot_counter = slot;
        
        while self.array[slot_counter as usize].hash != hash &&
              self.array[slot_counter as usize].state == EntryState::Occupied {

            slot_counter = (slot_counter + 1) %  size;

            if slot_counter == slot {
                return Err(InsertError::IsFull);
            }
        }

        let entry = &mut self.array[slot_counter as usize];

        entry.hash = hash;

        match entry.state {
            EntryState::Empty => {
                entry.value = v;
                entry.state = EntryState::Occupied;
                Ok(None)
            },
            EntryState::Occupied => {
                let old = entry.value;
                entry.value = v;
                entry.state = EntryState::Occupied;
                Ok(Some(old))
            },
        }
    }

    pub fn get(&self, k: K) -> Option<V> {

        let (slot, hash) = self.get_slot_and_hash(k);

        let size = self.array.len() as u64;
        let mut slot_counter = slot;
        

        while self.array[slot_counter as usize].hash != hash &&
              self.array[slot_counter as usize].state == EntryState::Occupied {

            slot_counter = (slot_counter + 1) % size;

            if slot_counter == slot {
                return None;
            }
        }

        let entry = &self.array[slot_counter as usize];

        match entry.state {
            EntryState::Empty => {
                None
            },
            EntryState::Occupied => {
                if hash != entry.hash {
                    panic!("Wrong hash stored here");
                }
                Some(entry.value)
            },
        }
    }

    fn get_slot_and_hash(&self, k: K) -> (u64, u64) {
        let hash = 0x8000_0000_0000_0000 | hash(k);
        (hash % self.array.len() as u64, hash)
    }
}