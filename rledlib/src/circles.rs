use rpiledbind::LedMatrix;

pub fn run() {
    let mut matrix = LedMatrix::new();
    let r: u8 = 0;
    let g: u8 = 0;
    let b: u8 = 255;

    let y: i32 = 16;

    for i in 0..100 {
        for x in 0..matrix.width() {
            matrix.clear();
            matrix.draw_circle(
                x,
                y,
                ((x as f32).sin() * ((matrix.width() / 2) as f32)) as i32,
                x as u8,
                i as u8,
                b,
            );
            matrix.swap_canvas();
        }
    }
}
