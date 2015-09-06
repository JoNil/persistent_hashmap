#![feature(plugin)]

#![plugin(clippy)]

#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss,
        non_ascii_literal, shadow_same, string_add, string_add_assign, unicode_not_nfc)]

extern crate persistent_hashmap;

use persistent_hashmap::{InsertError, PersistentHashmap};

#[test]
fn test() {
    {
        let mut db = PersistentHashmap::<&str, u8>::new("test.db", 10).unwrap();

        assert_eq!(db.insert("a", 1), Ok(None));
        assert_eq!(db.insert("b", 2), Ok(None));
        assert_eq!(db.insert("c", 3), Ok(None));
    }
    {
        let db = PersistentHashmap::<&str, u8>::open("test.db").unwrap();

        assert_eq!(db.get("a"), Some(1));
        assert_eq!(db.get("b"), Some(2));
        assert_eq!(db.get("c"), Some(3));
    }
}
#[test]
fn test2() {
    {
        let mut db = PersistentHashmap::<&str, u8>::new("test2.db", 5).unwrap();

        assert_eq!(db.insert("a", 1), Ok(None));
        assert_eq!(db.insert("b", 2), Ok(None));
        assert_eq!(db.insert("c", 3), Ok(None));
        assert_eq!(db.insert("d", 4), Ok(None));
        assert_eq!(db.insert("e", 5), Ok(None));
        assert_eq!(db.insert("e", 6), Ok(Some(5)));

        assert_eq!(db.insert("f", 6), Err(InsertError::IsFull));
    }
    {
        let db = PersistentHashmap::<&str, u8>::open("test2.db").unwrap();

        assert_eq!(db.get("a"), Some(1));
        assert_eq!(db.get("b"), Some(2));
        assert_eq!(db.get("c"), Some(3));
        assert_eq!(db.get("d"), Some(4));
        assert_eq!(db.get("e"), Some(6));
        assert_eq!(db.get("f"), None);
    }
}

#[test]
fn test3() {
    {
        let mut db = PersistentHashmap::<&str, u8>::new("test3.db", 5).unwrap();

        assert_eq!(db.insert("1", 1), Ok(None));
        assert_eq!(db.insert("2", 2), Ok(None));
        assert_eq!(db.insert("3", 3), Ok(None));
        assert_eq!(db.insert("4", 4), Ok(None));
        assert_eq!(db.insert("5", 5), Ok(None));
        assert_eq!(db.insert("5", 6), Ok(Some(5)));

        assert_eq!(db.insert("6", 6), Err(InsertError::IsFull));
    }
    {
        let db = PersistentHashmap::<&str, u8>::open("test3.db").unwrap();

        assert_eq!(db.get("1"), Some(1));
        assert_eq!(db.get("2"), Some(2));
        assert_eq!(db.get("3"), Some(3));
        assert_eq!(db.get("4"), Some(4));
        assert_eq!(db.get("5"), Some(6));
        assert_eq!(db.get("6"), None);
    }
}

#[test]
fn test4() {

    let capacity = 10*1024*1024;
    let size = 1024*1024;

    {
        let mut db = PersistentHashmap::<u64, u64>::new("test4.db", capacity).unwrap();

        for i in 0..size {
            assert_eq!(db.insert(&i, i), Ok(None));    
        }
    }
    {
        let db = PersistentHashmap::<u64, u64>::open("test4.db").unwrap();

        for i in 0..size {
            assert_eq!(db.get(&i), Some(i));    
        }
    }
}

#[test]
fn test5() {

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct Data {
        a: u64,
        b: u64,
        c: [u8; 3],
    }

    {
        let mut db = PersistentHashmap::<&str, Data>::new("test5.db", 5).unwrap();

        assert_eq!(db.insert("1", Data { a: 1, b: 2, c: *b"he1" }), Ok(None));
        assert_eq!(db.insert("2", Data { a: 2, b: 3, c: *b"he2" }), Ok(None));
        assert_eq!(db.insert("3", Data { a: 3, b: 4, c: *b"he3" }), Ok(None));
        assert_eq!(db.insert("4", Data { a: 4, b: 5, c: *b"he4" }), Ok(None));
        assert_eq!(db.insert("5", Data { a: 5, b: 6, c: *b"he5" }), Ok(None));
    }
    {
        let db = PersistentHashmap::<&str, Data>::open("test5.db").unwrap();

        assert_eq!(db.get("1"), Some(Data { a: 1, b: 2, c: *b"he1" }));
        assert_eq!(db.get("2"), Some(Data { a: 2, b: 3, c: *b"he2" }));
        assert_eq!(db.get("3"), Some(Data { a: 3, b: 4, c: *b"he3" }));
        assert_eq!(db.get("4"), Some(Data { a: 4, b: 5, c: *b"he4" }));
        assert_eq!(db.get("5"), Some(Data { a: 5, b: 6, c: *b"he5" }));
    }
}

#[test]
fn test6() {
    {
        let mut db = PersistentHashmap::<String, u32>::new("test6.db", 5).unwrap();

        assert_eq!(db.insert("1", 1), Ok(None));
        assert_eq!(db.insert("2", 2), Ok(None));
        assert_eq!(db.insert("3", 3), Ok(None));
        assert_eq!(db.insert("4", 4), Ok(None));
        assert_eq!(db.insert("5", 5), Ok(None));
    }
    {
        let db = PersistentHashmap::<String, u32>::open("test6.db").unwrap();

        assert_eq!(db.get("1"), Some(1));
        assert_eq!(db.get("2"), Some(2));
        assert_eq!(db.get("3"), Some(3));
        assert_eq!(db.get("4"), Some(4));
        assert_eq!(db.get("5"), Some(5));
    }
}

#[test]
fn test7() {
    {
        let mut db = PersistentHashmap::<str, u32>::new("test7.db", 5).unwrap();

        assert_eq!(db.insert("1", 1), Ok(None));
        assert_eq!(db.insert("2", 2), Ok(None));
        assert_eq!(db.insert("3", 3), Ok(None));
        assert_eq!(db.insert("4", 4), Ok(None));
        assert_eq!(db.insert("5", 5), Ok(None));
    }
    {
        let db = PersistentHashmap::<str, u32>::open("test7.db").unwrap();

        assert_eq!(db.get("1"), Some(1));
        assert_eq!(db.get("2"), Some(2));
        assert_eq!(db.get("3"), Some(3));
        assert_eq!(db.get("4"), Some(4));
        assert_eq!(db.get("5"), Some(5));
    }
}