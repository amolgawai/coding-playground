use std::{collections::HashMap, fs};
pub struct Database {
    map: HashMap<String, String>,
}

impl Database {
    // pub fn new(db_str:&String) -> Result<Database, std::io::Error> {
    pub fn new(db_str: &String) -> Database {
        // let contents = fs::read_to_string("kv.db")?;
        let mut db_map = HashMap::new();
        // for a_line in contents.lines() {
        for a_line in db_str.lines() {
            let mut chunk = a_line.splitn(2, "\t");
            let key = chunk.next().expect("corrupt database");
            let value = chunk.next().expect("corrupt database");
            // db_map.insert(String::from(key), String::from(value));
            db_map.insert(key.to_owned(), value.to_owned());
        }
        // Ok(Database { map: db_map })
        Database { map: db_map }
    }

    pub fn map(self: &Self) -> HashMap<String, String> {
        self.map.clone()
    }

    pub fn add(self: &mut Self, key: &str, value: &str) {
        self.map.insert(key.to_owned(), value.to_owned());
    }

    pub fn write_to_file(self: Self, f_path: &str) -> std::io::Result<()> {
        let mut db_str = String::new();
        for (key, value) in self.map.iter() {
            let db_line = format!("{}\t{}\n", key, value);
            db_str.push_str(&db_line);
        }

        fs::write(f_path, db_str)
    }
}
