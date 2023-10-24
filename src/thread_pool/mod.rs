//! 该模块提供了各种线程池，所有线程池都应该实现 “ThreadPool” 特性。

use crate::Result;

mod naive;
mod rayon;
mod shared_queue;
