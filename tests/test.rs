extern crate persistent_hashmap;

use persistent_hashmap::PersistentHashmap;

#[test]
fn test() {
    {
        PersistentHashmap::<u64>::new("test.db", 1024).unwrap();
    }
}
