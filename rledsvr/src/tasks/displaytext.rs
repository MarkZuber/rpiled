use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, LoopState};
use std::thread;
use std::time;

pub struct DisplayTextTask {
    to_display: String,
}

impl DisplayTextTask {
    pub fn new(to_display: &str) -> Self {
        Self {
            to_display: to_display.to_string(),
        }
    }
}

impl Cancellable for DisplayTextTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        eprintln!("displaying some text: {}", self.to_display);
        thread::sleep(time::Duration::from_secs(2));
        Ok(LoopState::Continue)
    }
}
