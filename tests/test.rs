extern crate persistent_hashmap;

use persistent_hashmap::PersistentHashmap;

#[test]
fn test() {
    {
        let db = PersistentHashmap::<str, u64>::new("test.db", 1024).unwrap();

        //db["a"] = 1;
        //db["b"] = 2;
        //db["c"] = 3;
    }
    {
        let db = PersistentHashmap::<str, u64>::open("test.db").unwrap();

        //assert_eq!(db["a"], 1);
        //assert_eq!(db["b"], 1);
        //assert_eq!(db["c"], 1);
    }
}
