use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, LoopState};
use rledlib::LedMatrix;

pub struct CirclesTask {
    matrix: LedMatrix,
    counter: u8,
    x: u8,
    y: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl CirclesTask {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            matrix: new LedMatrix(),
            counter: 0,
            x: 0_u8,
            y: 16_u8,
            r,
            g,
            b
        }
    }
}

impl Cancellable for CirclesTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        self.matrix.clear();
        self.matrix.draw_circle(
            self.x,
            self.y,
            ((self.x as f32).sin() * ((self.matrix.width() / 2) as f32)) as i32,
            // self.x as u8,
            // self.i as u8,
            // self.b,
            self.r,
            self.g,
            self.b
        );
        self.matrix.swap_canvas();

        self.x = self.x + 1;
        if self.x > self.matrix.width() {
            self.x = 0;
        }
        self.i = self.i + 1;
        if self.i > 100 {
            self.i = 0;
        }

        Ok(LoopState::Continue)
    }
}
