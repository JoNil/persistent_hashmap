extern crate persistent_hashmap;

use persistent_hashmap::PersistentHashmap;

#[test]
fn test() {
    {
        let mut db = PersistentHashmap::<&str, u64>::new("test.db", 1024).unwrap();

        db.insert("a", 1);
        db.insert("b", 2);
        db.insert("c", 3);
    }
    {
        let db = PersistentHashmap::<&str, u64>::open("test.db").unwrap();

        assert_eq!(db.get("a"), Some(1));
        assert_eq!(db.get("b"), Some(2));
        assert_eq!(db.get("c"), Some(3));
    }
}
