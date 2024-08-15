use std::thread::ThreadId;

pub struct WorkerToMain {
    pub steps: u32,
    pub thread_id: ThreadId,
}

#[derive(PartialEq)]
pub enum MainToWorker {
    KeepLooking,
    Exit,
}