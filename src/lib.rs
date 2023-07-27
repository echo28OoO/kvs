use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { map: HashMap::new() }
    }

    // set key
    pub fn set(&mut self, key: String, vlaue: String) {
        self.map.insert(key, vlaue);
    }

    // get key
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    // remove key
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

