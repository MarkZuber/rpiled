mod circles;
mod displaytext;

use self::displaytext::DisplayTextTask;
use crate::tasks::circles::CirclesTask;
use rpiledbind::MatrixHolder;

use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, Handle, LoopState};
use core::TaskMessage;

pub fn spawn_task_from_message(matrix: &MatrixHolder, msg: &TaskMessage) -> Handle<TaskError> {
    return match msg {
        TaskMessage::DisplayText { text_blocks } => {
            DisplayTextTask::new(matrix, text_blocks.clone()).spawn()
        }
        TaskMessage::Circles { r, g, b } => CirclesTask::new(matrix, *r, *g, *b).spawn(),
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
