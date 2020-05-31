use rpiledbind::bindings::*;

pub mod simple;

pub struct LedMatrix {
    matrix: *mut RGBLedMatrix,
    offscreen_canvas: *mut LedCanvas,
    width: i32,
    height: i32,
}

impl LedMatrix {
    pub fn new() -> Self {
        unsafe {
            let matrix = led_matrix_create(32, 2, 1);
            let offscreen_canvas = led_matrix_create_offscreen_canvas(matrix);
            let mut width: i32 = 0;
            let mut height: i32 = 0;
            led_canvas_get_size(offscreen_canvas, &mut width, &mut height);
            LedMatrix {
                matrix,
                offscreen_canvas,
                width,
                height,
            }
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        unsafe {
            led_canvas_set_pixel(self.offscreen_canvas, x, y, r, g, b);
        }
    }

    pub fn swap_canvas(&mut self) {
        /* Now, we swap the canvas. We give swap_on_vsync the buffer we
         * just have drawn into, and wait until the next vsync happens.
         * we get back the unused buffer to which we'll draw in the next
         * iteration.
         */

        unsafe {
            self.offscreen_canvas = led_matrix_swap_on_vsync(self.matrix, self.offscreen_canvas);
        }
    }

    pub fn close(&mut self) {
        unsafe {
            led_matrix_delete(self.matrix);
        }
    }
}

impl Drop for LedMatrix {
    fn drop(&mut self) {
        self.close();
    }
}
