use std::collections::HashMap;
pub struct KVStore {
    map: HashMap<String, String>,
}

impl KVStore {
    // pub fn new(db_str:&String) -> Result<Database, std::io::Error> {
    pub fn from_str(db_str: &str) -> KVStore {
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
        KVStore { map: db_map }
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_owned(), value.to_owned());
        self.map.insert(key.to_uppercase(), value.to_owned());
    }

    pub fn to_str(&self) -> String {
        let mut db_str = String::new();
        for (key, value) in self.map.iter() {
            // let db_line = format!("{}\t{}\n", key, value);
            // db_str.push_str(&db_line);
            db_str.push_str(key);
            db_str.push('\t');
            db_str.push_str(value);
            db_str.push('\n');
        }

        return db_str;
    }
}
