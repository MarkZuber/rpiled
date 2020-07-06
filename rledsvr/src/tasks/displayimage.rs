use crate::taskmgr::TaskError;
use core::jobs::{Cancellable, LoopState};
use image::imageops::FilterType;
use image::{imageops, Pixel};
use rpiledbind::MatrixHolder;

pub struct DisplayImageTask {
    matrix: MatrixHolder,
    image_path: String,
}

impl DisplayImageTask {
    pub fn new(matrix: &MatrixHolder, image_path: &str) -> Self {
        Self {
            matrix: matrix.clone(),
            image_path: image_path.to_string(),
        }
    }
}

impl Cancellable for DisplayImageTask {
    type Error = TaskError;
    fn for_each(&mut self) -> Result<LoopState, Self::Error> {
        let mut matrix = self.matrix.lock_matrix();
        matrix.clear();

        let img = image::open(&self.image_path).unwrap();
        let scale_img = imageops::resize(
            &img,
            matrix.width() as u32,
            matrix.height() as u32,
            FilterType::Triangle,
        );

        for pixel in scale_img.enumerate_pixels() {
            let channels = pixel.2.channels();
            matrix.set_pixel(
                pixel.0 as i32,
                pixel.1 as i32,
                channels[0],
                channels[1],
                channels[2],
            );
        }

        matrix.swap_canvas();
        Ok(LoopState::Break)
    }
}
