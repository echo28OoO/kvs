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
