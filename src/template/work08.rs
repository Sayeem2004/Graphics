// Imports
use crate::algorithm::{line, shape};
use crate::format::{file, image::Image, matrix::Matrix, pixel::Pixel};
use crate::script::parse;
use std::fs;

/// Function that creates the gradient sphere image
pub fn gradient_sphere(mode: i32) {
    // Variable declarations
    let mut img: Image = Image::new_dimension(750, 750);
    let mut poly: Matrix = Matrix::new_matrix();
    let mut cord: Matrix = Matrix::new_transformation();
    let trans: Matrix = Matrix::translate(375.0, 375.0, 0.0);

    // Creating sphere and translating
    cord.right_transform(&trans);
    shape::add_sphere(&mut poly, (0.0, 0.0, 0.0), 300.0, 100);
    poly.left_transform(&cord);

    // Drawing sphere with gradient
    for i in 0..poly.data.len() {
        if (i % 3 == 2) {
            line::scanline(
                &poly,
                i as i32,
                &mut img,
                Pixel::new_value(
                    ((poly.data[i][0] + poly.data[i][1]) / 1200_f32 * 256_f32) as u8,
                    0,
                    0,
                ),
            )
        }
    }

    // Saving image
    file::create_ppm_ascii("image/ppm/w08_gradient.ppm", &img, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w08_gradient.ppm");
    }
}

/// Function that creates all images from work 08
pub fn create_work08_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating shapes image test
    parse::parse("data/w08/w08_script", 500, mode);

    // Creating gradient sphere image
    gradient_sphere(mode);
}
