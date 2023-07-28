use std::collections::HashMap;


#[derive(Default)]
/// KvStore basic
pub struct KvStore {
    map: HashMap<String, String>,
}

/// KvStore 接口
impl KvStore {
    /// 创建一个 'KvStore {map}'
    pub fn new() -> KvStore {
        KvStore { map: HashMap::new() }
    }

    /// 向 KvStore map 存入新的 key - value
    pub fn set(&mut self, key: String, vlaue: String) {
        self.map.insert(key, vlaue);
    }

    /// 根据 key 从 KvStore 中获取相应的 value
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// 根据 key 从 KvStore 中删除相应的 key - value
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

