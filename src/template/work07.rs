// Imports
use crate::algorithm::shape;
use crate::format::{constant, file, image::Image, matrix::Matrix};
use crate::script::parse;
use std::{f32::consts::PI, fs};

/// Function that creates the spiral unraveling gif
pub fn spiral_chain(mode: i32) {
    // Variable declarations
    let mut cord: Matrix = Matrix::new_transformation();
    let mut poly: Matrix = Matrix::new_matrix();
    let mut img: Image = Image::new_dimension(750, 750);
    let rotx: Matrix = Matrix::rotate_degree(90.0, "x");

    // Initial shift of coordinate plane
    let mut trans: Matrix = Matrix::translate(375.0, 375.0, 0.0);
    cord.right_transform(&trans);

    // Creating initial spiral
    for i in 0..400 {
        // Variable declarations
        let a: f32 = 10.0;
        let t: f32 = (i as f32) / 8.0;
        let r: f32 = a * t;
        let rotz: Matrix = Matrix::rotate_radian(PI / 2.0 + t, "z");

        // Shifting to torus center
        trans = Matrix::translate(r * f32::cos(t), r * f32::sin(t), 0.0);
        cord.right_transform(&trans);

        // Drawing torus
        shape::add_torus(&mut poly, (0.0, 0.0, 0.0), 2.0, 10.0, 10);
        if (i % 2 == 0) {
            poly.left_transform(&rotx);
        }
        poly.left_transform(&rotz);
        poly.left_transform(&cord);
        poly.draw_triangles_xy(&mut img, constant::WHITE_PIXEL, &constant::ZVIEW);
        poly.clear();

        // Reverting
        trans = Matrix::translate(-r * f32::cos(t), -r * f32::sin(t), 0.0);
        cord.right_transform(&trans);
    }

    // Saving image and displaying
    file::create_ppm_ascii("image/ppm/w07_spiralchain.ppm", &img, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w07_spiralchain.ppm");
    }
}

/// Function that creates the olympics symbol image
pub fn olympics(mode: i32) {
    // Variable declarations
    let mut cord: Matrix = Matrix::new_transformation();
    let mut poly: Matrix = Matrix::new_matrix();
    let mut img: Image = Image::new_dimension(800, 800);
    img.fill(constant::WHITE_PIXEL);
    let mut trans: Matrix = Matrix::translate(130.0, 460.0, 0.0);
    let rot: Matrix = Matrix::rotate_degree(90.0, "x");

    // Blue ring
    cord.right_transform(&trans);
    shape::add_torus(&mut poly, (0.0, 0.0, 0.0), 10.0, 120.0, 300);
    poly.left_transform(&rot);
    poly.left_transform(&cord);
    poly.draw_triangles_xy(&mut img, constant::AQUA_PIXEL, &constant::ZVIEW);
    poly.clear();

    // Yellow ring
    trans = Matrix::translate(135.0, -120.0, 0.0);
    cord.right_transform(&trans);
    shape::add_torus(&mut poly, (0.0, 0.0, 0.0), 10.0, 120.0, 300);
    poly.left_transform(&rot);
    poly.left_transform(&cord);
    poly.draw_triangles_xy(&mut img, constant::YELLOW_PIXEL, &constant::ZVIEW);
    poly.clear();

    // Black ring
    trans = Matrix::translate(135.0, 120.0, 0.0);
    cord.right_transform(&trans);
    shape::add_torus(&mut poly, (0.0, 0.0, 0.0), 10.0, 120.0, 300);
    poly.left_transform(&rot);
    poly.left_transform(&cord);
    poly.draw_triangles_xy(&mut img, constant::BLACK_PIXEL, &constant::ZVIEW);
    poly.clear();

    // Green ring
    trans = Matrix::translate(135.0, -120.0, 0.0);
    cord.right_transform(&trans);
    shape::add_torus(&mut poly, (0.0, 0.0, 0.0), 10.0, 120.0, 300);
    poly.left_transform(&rot);
    poly.left_transform(&cord);
    poly.draw_triangles_xy(&mut img, constant::LIME_PIXEL, &constant::ZVIEW);
    poly.clear();

    // Red ring
    trans = Matrix::translate(135.0, 120.0, 0.0);
    cord.right_transform(&trans);
    shape::add_torus(&mut poly, (0.0, 0.0, 0.0), 10.0, 120.0, 300);
    poly.left_transform(&rot);
    poly.left_transform(&cord);
    poly.draw_triangles_xy(&mut img, constant::RED_PIXEL, &constant::ZVIEW);
    poly.clear();

    // Saving image and displaying
    file::create_ppm_ascii("image/ppm/w07_olympics.ppm", &img, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w07_olympics.ppm");
    }
}

/// Function that creates all images from work 07
pub fn create_work07_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating robot image test
    parse::parse("data/w07/w07_robot", 750, mode);

    // Creating spiral unraveling gif
    spiral_chain(mode);

    // Creating olympics symbol image
    olympics(mode);

    // Creating umbrella image
    parse::parse("data/w07/w07_umbrella", 750, mode);
}
