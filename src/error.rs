use failure::Fail;
use std::io;
use std::string::FromUtf8Error;

/// kvs 的 Error 处理
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error.
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),
    /// 序列化或者反序列化错误
    #[fail(display = "serde_json error: {}", _0)]
    Serde(#[cause] serde_json::Error),
    /// 删除没有的键值错误
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// 意料之外的命令错误
    /// log 日志或者程序 bug 等等
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
    /// key 或 value 是无效的 UTF-8 序列
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),
    /// Sled 错误
    #[fail(display = "sled error: {}", _0)]
    Sled(#[cause] sled::Error),
    /// 错误消息
    #[fail(display = "{}", _0)]
    StringError(String),
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

impl From<FromUtf8Error> for KvsError {
    fn from(err: FromUtf8Error) -> KvsError {
        KvsError::Utf8(err)
    }
}

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        KvsError::Sled(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
