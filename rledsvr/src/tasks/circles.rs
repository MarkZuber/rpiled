use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, LoopState};
use rpiledbind::MatrixHolder;

pub struct CirclesTask {
    matrix: MatrixHolder,
    counter: u8,
    x: i32,
    y: i32,
    r: u8,
    g: u8,
    b: u8,
}

impl CirclesTask {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            matrix: MatrixHolder::new(),
            counter: 0,
            x: 0,
            y: 16,
            r,
            g,
            b,
        }
    }
}

impl Cancellable for CirclesTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        let mut matrix = self.matrix.lock_matrix();
        matrix.clear();
        matrix.draw_circle(
            self.x,
            self.y,
            ((self.x as f32).sin() * ((matrix.width() / 2) as f32)) as i32,
            self.r,
            self.g,
            self.b,
        );
        matrix.swap_canvas();

        self.x = self.x + 1;
        if self.x > matrix.width() {
            self.x = 0;
        }
        self.counter = self.counter + 1;
        if self.counter > 100 {
            self.counter = 0;
        }

        Ok(LoopState::Continue)
    }
}
