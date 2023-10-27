//! 该模块提供了各种线程池，所有线程池都应该实现 “ThreadPool” 特性。

use crate::Result;

mod naive;
mod rayon;
mod shared_queue;

/// 所有线程池都应该实现的特征。
pub trait ThreadPool {
    /// 创建一个新的线程池，立即生成指定数量的线程
    /// 
    /// 如果任何线程未能生成，则返回错误。所有先前生成的线程都被终止
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;
    
    /// 将函数生成到线程池中。
    /// 
    /// 生成总是成功，但如果函数出现紧急情况，线程池将继续使用相同的
    /// 线程进行操作，线程计数没有减少，线程池也没有被破坏、损坏或失效
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
