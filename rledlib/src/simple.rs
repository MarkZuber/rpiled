use rpiledbind::LedMatrix;

pub fn run() {
    let mut matrix = LedMatrix::new();
    for i in 0..1000 {
        for y in 0..matrix.height() {
            for x in 0..matrix.width() {
                matrix.set_pixel(x, y, i as u8, x as u8, y as u8);
            }
        }
        matrix.swap_canvas();
    }
}
