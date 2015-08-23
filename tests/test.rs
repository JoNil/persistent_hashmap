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
