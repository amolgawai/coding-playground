mod database;
use std::{env, fs, path::Path};
fn main() {
    let mut args = env::args().skip(1);
    let key = args.next().expect("Please provide a key");
    let value = args.next().expect("Please provide a value");
    // print!("key {}, value {}", key, value);

    // let contents = format!("{}\t{}\n", key, value);
    // let write_result = fs::write("kv.db", contents);

    // match write_result {
    //     Ok(()) => {}
    //     Err(e) => println!("Error writing db - {}", e),
    // }

    let db_path = Path::new("kv.db");
    let mut db_str = String::new();

    if db_path.exists() {
        db_str = fs::read_to_string("kv.db").expect("no database found");
    }

    // let db = Database::new();
    let mut db = database::Database::new(&db_str);
    db.add(&key[..], &value[..]);
    println!("{:?}", db.map());

    db.write_to_file(db_path.to_str().unwrap())
        .expect("Error writing db file");
}
