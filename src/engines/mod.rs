//! 该模块提供各种 key value 存储引擎

use crate::Result;

/// key value 存储引擎的 trait
pub trait KvsEngine {
    /// 设置 key(string) 的 value (string)
    /// 
    /// 如果 key 已经存在，则会覆盖 value
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// 获取 key 的 value
    /// 
    /// 如果 key 不存在，则返回 None
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// 删除 key
    /// 
    /// 如果 key 不存在，则返回错误 KvsError::KeyNotFound
    fn remove(&mut self, key: String) -> Result<()>;
}

mod kvs;
mod sled;

pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;
