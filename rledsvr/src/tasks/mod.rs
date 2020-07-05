mod circles;
mod displaytext;
mod scrolltext;

use self::displaytext::DisplayTextTask;
use crate::tasks::circles::CirclesTask;
use crate::tasks::scrolltext::ScrollTextTask;
use rpiledbind::MatrixHolder;

use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, Handle, LoopState};
use core::TaskMessage;

pub fn spawn_task_from_message(matrix: &MatrixHolder, msg: &TaskMessage) -> Handle<TaskError> {
    return match msg {
        TaskMessage::DisplayText { text_blocks } => {
            DisplayTextTask::new(matrix, text_blocks.clone()).spawn()
        }
        TaskMessage::ScrollText {
            text_blocks,
            x_delta,
            y_delta,
            num_steps,
            frame_millis,
        } => ScrollTextTask::new(
            matrix,
            text_blocks.clone(),
            *x_delta,
            *y_delta,
            *num_steps,
            *frame_millis,
        )
        .spawn(),
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
