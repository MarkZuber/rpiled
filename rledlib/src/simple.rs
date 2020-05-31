// use rpiledbind::bindings::*;
use crate::LedMatrix;

pub fn run() {
    let mut matrix = LedMatrix::new();
    for i in 0..1000 {
        for y in 0..matrix.height {
            for x in 0..matrix.width {
                matrix.set_pixel(x, y, i as u8, x as u8, y as u8);
            }
        }
        matrix.swap_canvas();
    }

    // let mut matrix = led_matrix_create(32, 2, 1);
    // let mut width: i32 = 0;
    // let mut height: i32 = 0;
    // let mut offscreen_canvas = led_matrix_create_offscreen_canvas(matrix);
    // led_canvas_get_size(offscreen_canvas, &mut width, &mut height);
    // for i in 0..1000 {
    //     for y in 0..height {
    //         for x in 0..width {
    //             led_canvas_set_pixel(offscreen_canvas, x, y, i as u8, x as u8, y as u8);
    //         }
    //     }

    //     let mut offscreen_canvas = led_matrix_swap_on_vsync(matrix, offscreen_canvas);
    // }
    // led_matrix_delete(matrix);
}
