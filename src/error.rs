use failure::Fail;
use std::io;

/// kvs 的 Error 处理
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error.
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// 序列化或者反序列化错误
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    /// 删除没有的键值错误
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// 意料之外的命令错误
    /// log 日志或者程序 bug 等等
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
