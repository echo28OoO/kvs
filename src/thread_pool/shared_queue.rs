use std::thread;

use super::ThreadPool;
use crate::Result;

use crossbeam::channel::{self, Receiver, Sender};

use log::{debug, error};

/// 注意事项：线程池不是使用 `catch_unwind` 实现的，因为
/// 它要求任务是 `UnwindSafe`。

/// 内部使用共享队列的线程池。
/// 
/// 如果生成的任务发生 panics，旧线程将被销毁，并创建一个新线程。
/// 当创建线程池后捕获到在操作系统级别创建线程的任何失败时，
/// 它的失败会线程池创建后被捕获。所以池中的线程数可以减少到零，
/// 然后讲任务派生到线程池会出现 panics
pub struct SharedQueueThreadPool {
    tx: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new (threads: u32) -> Result<Self> {
        let (tx, rx) = channel::unbounded::<Box<dyn FnOnce() + Send + 'static>>();
        for _ in 0..threads {
            ..
        }
    }
}

#[derive(Clone)]
struct TaskReceiver(Receiver<Box<dyn FnOnce() + Send + 'static>>);

impl Drop for TaskReceiver {
    fn drop(&mut self) {
        if thread::panicking() {
            let rx = self.clone();
            if let Err(e) = thread::Builder::new().spawn(move || run_tasks(rx)) {
                error!("Failed to spawn a thread: {}", e);
            }
        }
    }
}

fn run_tasks(rx: TaskReceiver) {
    loop {
        match rx.0.recv() {
            Ok(task) => {
                task();
            }
            Err(_) => debug!("Thread exits because th thread pool is destroyed."),
        }
    }
}