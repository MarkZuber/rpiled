use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, LoopState};
use rpiledbind::MatrixHolder;

pub struct CirclesTask {
    matrix: MatrixHolder,
    i: i32,
    j: i32,
    k: i32,
    step_value: i32,
    counter: u8,
    x: i32,
    y: i32,
}

impl CirclesTask {
    pub fn new(matrix: &MatrixHolder) -> Self {
        Self {
            matrix: matrix.clone(),
            i: 0,
            j: 0,
            k: 0,
            step_value: 50,
            counter: 0,
            x: 0,
            y: 16,
        }
    }
}

impl Cancellable for CirclesTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        let mut matrix = self.matrix.lock_matrix();
        matrix.clear();

        let r = self.i as u8;
        let g = self.j as u8;
        let b = self.k as u8;

        matrix.draw_circle(
            self.x,
            self.y,
            ((self.x as f32).sin() * ((matrix.width() / 2) as f32)) as i32,
            r,
            g,
            b,
        );
        matrix.swap_canvas();

        self.x = self.x + 1;
        if self.x > matrix.width() {
            self.x = 0;
        }
        self.counter = self.counter + 1;
        if self.counter > 100 {
            self.counter = 0;

            self.k = self.k + self.step_value;
            if self.k > 255 {
                self.k = 0;
                self.j = self.j + self.step_value;
                if self.j > 255 {
                    self.j = 0;
                    self.i = self.i + self.step_value;
                    if self.i > 255 {
                        self.i = 0;
                    }
                }
            }
        }

        Ok(LoopState::Continue)
    }
}
