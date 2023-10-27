use std::thread;

use super::ThreadPool;
use crate::Result;

/// 它实际上不是一个线程池。每次调用`spawn`方法时，它会生成一个新线程

pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {
    fn new(_threads: u32) -> Result<Self> {
        Ok(NaiveThreadPool)
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(job);
    }
}
