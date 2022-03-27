// Imports
use crate::algorithm::curve;
use crate::format::{constant, file, image::Image, matrix::Matrix};
use crate::script::parse;
use std::f32::consts::PI;
use std::fs;

/// Function that creates the optical illusion image
fn optical() {
    // Variable declarations
    let mut img: Image = Image::new_dimension(750, 750);
    let mut crv: Matrix = Matrix::new_matrix();
    crv.add_col(&vec![30.0, 750.0, 0.0, 1.0]);
    crv.add_col(&vec![37.5, 250.0, 0.0, 1.0]);
    crv.add_col(&vec![300.0, 225.0, 0.0, 1.0]);
    crv.add_col(&vec![375.0, 375.0, 0.0, 1.0]);
    let mut trans: Matrix = Matrix::new_transformation();
    trans.translate(-375.0, -375.0, 0.0);
    trans.rotate_degree(6.0, "z");
    trans.translate(375.0, 375.0, 0.0);
    let mut edge: Matrix = Matrix::new_matrix();

    // Iterating over rotation
    for _ in 0..60 {
        // Drawing curve
        curve::add_bezier(
            &mut edge,
            crv.data[0][0],
            crv.data[0][1],
            crv.data[1][0],
            crv.data[1][1],
            crv.data[2][0],
            crv.data[2][1],
            crv.data[3][0],
            crv.data[3][1],
            100,
        );

        // Updating points
        crv.matrix_transform(&trans);
    }

    // Drawing lines and filling in certain sections
    edge.draw_lines_xy(&mut img, constant::YELLOW_PIXEL);
    for i in 0..31 {
        img.flood_xy(
            (375.0 + 374.0 * f32::cos(2.0 * PI * i as f32 / 30.0)) as i32,
            (375.0 + 374.0 * f32::sin(2.0 * PI * i as f32 / 30.0)) as i32,
            constant::YELLOW_PIXEL,
            constant::WHITE_PIXEL,
        );
    }

    // Final touches and saving image
    img.replace(constant::YELLOW_PIXEL, constant::BLACK_PIXEL);
    file::create_ppm_ascii("image/ppm/w04_optical.ppm", &img, 0);
}

/// Function that creates all images from work 04
pub fn create_work04_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating face image
    parse::parse("data/w04/w04_script", 500, mode);

    // Creating optical illusion image
    optical();
    if (mode == 0) {
        file::open_image("image/ppm/w04_optical.ppm");
    }

    // Creating girl image
    parse::parse("data/w04/w04_girl", 750, mode);

    // Creating cat image
    parse::parse("data/w04/w04_cat", 750, mode);
}
