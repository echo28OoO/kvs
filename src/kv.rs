use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsError, Result};
use std::ffi::OsStr;

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

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

/// 代表命令的 struct
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}
