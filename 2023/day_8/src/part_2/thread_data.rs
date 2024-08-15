use std::{sync::mpsc::Sender, thread::JoinHandle};

use crate::part_2::thread_communication::*;

pub struct WorkerData {
    pub thread_handle: JoinHandle<()>,
    pub start_point: String,
    pub communication_to_worker: Sender<MainToWorker>,
    pub last_found_steps: u32,
    pub waiting_for_answer: bool,
}