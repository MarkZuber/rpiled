use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, LoopState};
use rpiledbind::FontHolder;
use rpiledbind::MatrixHolder;

pub struct DisplayTextTask {
    matrix: MatrixHolder,
    font: FontHolder,
    to_display: String,
    x: i32,
    y: i32,
    r: u8,
    g: u8,
    b: u8,
}

impl DisplayTextTask {
    pub fn new(font_path: &str, to_display: &str, x: i32, y: i32, r: u8, g: u8, b: u8) -> Self {
        Self {
            matrix: MatrixHolder::new(),
            font: FontHolder::new(font_path),
            to_display: to_display.to_string(),
            x,
            y,
            r,
            g,
            b,
        }
    }
}

impl Cancellable for DisplayTextTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        let mut matrix = self.matrix.lock_matrix();
        let font = self.font.lock_font();
        let kerning_offset: i32 = 0;
        matrix.draw_text(
            &font,
            self.x,
            self.y,
            self.r,
            self.g,
            self.b,
            &self.to_display,
            kerning_offset,
        );

        matrix.swap_canvas();

        Ok(LoopState::Continue)
    }
}
