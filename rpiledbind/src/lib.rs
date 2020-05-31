#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod bindings;

use crate::bindings::*;
use std::ffi::CString;

/// LED Matrix Controller to enable communication with the ADAFruit RGB HAT on the
/// Raspberry PI.
pub struct LedMatrix {
    matrix: *mut RGBLedMatrix,
    offscreen_canvas: *mut LedCanvas,
    width: i32,
    height: i32,
}

pub struct MatrixFont {
    led_font: *mut LedFont,
}

impl MatrixFont {
    pub fn new(font_file_path: &str) -> Self {
        let font_path_c = CString::new(String::from(font_file_path)).expect("CString new failed");
        unsafe {
            let led_font = load_font(font_path_c.as_ptr());
            MatrixFont { led_font }
        }
    }

    fn internal(&self) -> *mut LedFont {
        self.led_font
    }

    fn delete(&mut self) {
        unsafe { delete_font(self.led_font) }
    }
}

impl Drop for MatrixFont {
    fn drop(&mut self) {
        self.delete();
    }
}

impl LedMatrix {
    /// Create a new LED Matrix.
    /// This matrix is assumed to be 32 rows with 2 chained parts (so 64x32)
    /// since that's the kind of board I have so keeping it to the default.
    pub fn new() -> Self {
        LedMatrix::new_explicit(32, 2)
    }

    /// Create a new LED Matrix with explicit parameters for rows and chained.
    pub fn new_explicit(rows: i32, chained: i32) -> Self {
        unsafe {
            let matrix = led_matrix_create(rows, chained, 1);
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

    /// Width of the display in pixels
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Height of the display in pixels
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Set a pixel at (x,y) to a color (r,g,b)
    /// Pixels are set to an offscreen canvas for buffering.
    /// To get the pixel(s) you draw to show up, you need to call
    /// [`swap_canvas`].
    pub fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        unsafe {
            led_canvas_set_pixel(self.offscreen_canvas, x, y, r, g, b);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            led_canvas_clear(self.offscreen_canvas);
        }
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8) {
        unsafe {
            led_canvas_fill(self.offscreen_canvas, r, g, b);
        }
    }

    pub fn get_brightness(&mut self) -> u8 {
        unsafe { led_matrix_get_brightness(self.matrix) }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe {
            led_matrix_set_brightness(self.matrix, brightness);
        }
    }

    pub fn draw_text(
        &mut self,
        font: &MatrixFont,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
        text: &str,
        kerning_offset: i32,
    ) -> i32 {
        let text_c = CString::new(String::from(text)).expect("CString new failed");

        unsafe {
            draw_text(
                self.offscreen_canvas,
                font.internal(),
                x,
                y,
                r,
                g,
                b,
                text_c.as_ptr(),
                kerning_offset,
            )
        }
    }

    pub fn vertical_draw_text(
        &mut self,
        font: &MatrixFont,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
        text: &str,
        kerning_offset: i32,
    ) -> i32 {
        let text_c = CString::new(String::from(text)).expect("CString new failed");

        unsafe {
            vertical_draw_text(
                self.offscreen_canvas,
                font.internal(),
                x,
                y,
                r,
                g,
                b,
                text_c.as_ptr(),
                kerning_offset,
            )
        }
    }

    pub fn draw_circle(&self, x: i32, y: i32, radius: i32, r: u8, g: u8, b: u8) {
        unsafe {
            draw_circle(self.offscreen_canvas, x, y, radius, r, g, b);
        }
    }

    pub fn draw_line(&self, x0: i32, y0: i32, x1: i32, y1: i32, r: u8, g: u8, b: u8) {
        unsafe {
            draw_line(self.offscreen_canvas, x0, y0, x1, y1, r, g, b);
        }
    }

    /// Flips the offscreen canvas to the display on vsync and get an
    /// unused buffer to draw into for the next frame.
    pub fn swap_canvas(&mut self) {
        unsafe {
            self.offscreen_canvas = led_matrix_swap_on_vsync(self.matrix, self.offscreen_canvas);
        }
    }

    /// Explicitly close the LED matrix and shut down.  This will also happen on Drop.
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
