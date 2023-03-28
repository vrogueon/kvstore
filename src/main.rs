// output
// key1\tvalue1\nkey2\tvalue2\n

use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Missing 'key' argument");
    let value = arguments.next().expect("Missing 'value' argument");
    print!("The key is '{}', and the value is '{}'", key, value);

    let mut database = Database::new().expect("creating db failed");
    database.insert(key, value);
    
    match database.flush() {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}

struct Database {
    map: HashMap<String, String>,
    flush: bool, 
}

impl Database {
    // this returns a database struct
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No Key");
            let value = chunks.next().expect("No Value");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map, flush: false  })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_fush(&self)
    }
}
 
impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_fush(self);
        }
    }
} 

fn do_fush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
