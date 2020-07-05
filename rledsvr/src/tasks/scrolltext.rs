use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, LoopState};
use core::TextBlock;
use rpiledbind::MatrixFont;
use rpiledbind::MatrixHolder;
use std::{thread, time};

pub struct ScrollTextTask {
    matrix: MatrixHolder,
    text_blocks: Vec<TextBlock>,
    x_delta: i32,
    y_delta: i32,
    num_steps: i32,
    current_step: i32,
    frame_millis: u64,
}

impl ScrollTextTask {
    pub fn new(
        matrix: &MatrixHolder,
        text_blocks: Vec<TextBlock>,
        x_delta: i32,
        y_delta: i32,
        num_steps: i32,
        frame_millis: u64,
    ) -> Self {
        Self {
            matrix: matrix.clone(),
            text_blocks: text_blocks,
            x_delta,
            y_delta,
            num_steps,
            current_step: 0,
            frame_millis,
        }
    }
}

impl Cancellable for ScrollTextTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        let mut matrix = self.matrix.lock_matrix();
        matrix.clear();

        let kerning_offset: i32 = 0;

        if self.current_step == self.num_steps {
            self.current_step = 0;

            // Reset the blocks to their starting positions
            for (_, block) in self.text_blocks.iter_mut().enumerate() {
                block.x = block.x - (self.num_steps * self.x_delta);
                block.y = block.y - (self.num_steps * self.y_delta);
            }
        }

        for (_, block) in self.text_blocks.iter_mut().enumerate() {
            let font = MatrixFont::new(&block.font_path);

            matrix.draw_text(
                &font,
                block.x,
                block.y,
                block.r,
                block.g,
                block.b,
                &block.text,
                kerning_offset,
            );

            block.x = block.x + self.x_delta;
            block.y = block.y + self.y_delta;
        }
        thread::sleep(time::Duration::from_millis(self.frame_millis));

        matrix.swap_canvas();

        self.current_step = self.current_step + 1;

        Ok(LoopState::Continue)
    }
}
