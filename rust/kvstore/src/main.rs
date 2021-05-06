mod kvstore;
use std::{env, fs, path::Path};
fn main() {
    let mut args = env::args().skip(1);
    let key = args.next().expect("Please provide a key");
    let value = args.next().expect("Please provide a value");

    let db_path = Path::new("kv.db");
    let mut db_str = String::new();

    if db_path.exists() {
        db_str = fs::read_to_string("kv.db").expect("no database found");
    }

    // let db = Database::new();
    let mut db = kvstore::KVStore::from_str(&db_str);
    db.add(&key[..], &value[..]);

    fs::write(db_path.to_str().unwrap(), db.to_str()).expect("Error writing to file");
}
