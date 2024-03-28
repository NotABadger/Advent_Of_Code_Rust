pub extern crate threadpool;
pub use threadpool::ThreadPool;
extern crate num_cpus;

pub fn initialize_threadpool() -> ThreadPool {
    ThreadPool::new(num_cpus::get())
}