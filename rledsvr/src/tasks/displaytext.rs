use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, LoopState};
use core::TextBlock;
use rpiledbind::MatrixFont;
use rpiledbind::MatrixHolder;

pub struct DisplayTextTask {
    matrix: MatrixHolder,
    text_blocks: Vec<TextBlock>,
}

impl DisplayTextTask {
    pub fn new(text_blocks: Vec<TextBlock>) -> Self {
        Self {
            matrix: MatrixHolder::new(),
            text_blocks: text_blocks,
        }
    }
}

impl Cancellable for DisplayTextTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        let mut matrix = self.matrix.lock_matrix();

        let kerning_offset: i32 = 0;

        for block in &self.text_blocks {
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
        }

        matrix.swap_canvas();

        Ok(LoopState::Continue)
    }
}
