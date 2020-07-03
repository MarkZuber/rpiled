mod circles;
mod displaytext;

use self::displaytext::DisplayTextTask;
use crate::tasks::circles::CirclesTask;

use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, Handle, LoopState};
use core::TaskMessage;

pub fn spawn_task_from_message(msg: &TaskMessage) -> Handle<TaskError> {
    let t = CirclesTask::new(1_u8, 2_u8, 3_u8);
    return match msg {
        TaskMessage::DisplayText { text } => DisplayTextTask::new(text).spawn(),
        TaskMessage::Circles { r, g, b } => DisplayTextTask::new("hello").spawn(), // CirclesTask::new(*r, *g, *b).spawn(),
        _ => EmptyTask::new().spawn(),
    };
}

pub struct EmptyTask {}
impl EmptyTask {
    pub fn new() -> Self {
        Self {}
    }
}
impl Cancellable for EmptyTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        Ok(LoopState::Break)
    }
}
