extern crate image;

mod circles;
mod displayimage;
mod displaytext;
mod scrolltext;

use self::displaytext::DisplayTextTask;
use crate::tasks::circles::CirclesTask;
use crate::tasks::displayimage::DisplayImageTask;
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
        TaskMessage::DisplayImage { image_path } => {
            DisplayImageTask::new(matrix, image_path).spawn()
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
        TaskMessage::Circles {} => CirclesTask::new(matrix).spawn(),
        _ => EmptyTask::new(matrix).spawn(),
    };
}

pub struct EmptyTask {
    matrix: MatrixHolder,
}

impl EmptyTask {
    pub fn new(matrix: &MatrixHolder) -> Self {
        Self {
            matrix: matrix.clone(),
        }
    }
}
impl Cancellable for EmptyTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        let mut matrix = self.matrix.lock_matrix();
        matrix.clear();
        matrix.swap_canvas();
        Ok(LoopState::Break)
    }
}
