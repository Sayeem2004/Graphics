// Imports
use crate::format::{parse, file, matrix::Matrix, image::Image, constant};
use std::fs;

/// Function that creates a tesseract given a side length
pub fn tesseract(side : f32) {
    // Checking to make sure side length is positive
    if (side <= 0.0) {
        eprintln!("Side length can not be 0 or negative, no image made");
        return;
    }

    // Variable declarations
    let mut edge1 : Matrix = Matrix::new_matrix();
    let mut trans1 : Matrix = Matrix::new_transformation();
    let mut edge2 : Matrix = Matrix::new_matrix();
    let mut trans2 : Matrix = Matrix::new_transformation();
    let mut edge3 : Matrix = Matrix::new_matrix();
    let mut img : Image = Image::new_dimension((side*3.0) as i32, (side*3.0) as i32);

    // First cube
    edge1.add_edge(0.0, 0.0, 0.0, side, 0.0, 0.0);
    edge1.add_edge(side, 0.0, 0.0, side, side, 0.0);
    edge1.add_edge(side, side, 0.0, 0.0, side, 0.0);
    edge1.add_edge(0.0, side, 0.0, 0.0, 0.0, 0.0);
    edge1.add_edge(0.0, 0.0, side, side, 0.0, side);
    edge1.add_edge(side, 0.0, side, side, side, side);
    edge1.add_edge(side, side, side, 0.0, side, side);
    edge1.add_edge(0.0, side, side, 0.0, 0.0, side);
    edge1.add_edge(0.0, 0.0, 0.0, 0.0, 0.0, side);
    edge1.add_edge(0.0, side, 0.0, 0.0, side, side);
    edge1.add_edge(side, side, 0.0, side, side, side);
    edge1.add_edge(side, 0.0, 0.0, side, 0.0, side);
    trans1.rotate_degree(45.0, "y");
    trans1.rotate_degree(45.0, "x");
    trans1.translate(side / 2.5, side * 1.1, 0.0);
    edge1.matrix_transform(&trans1);

    // Second cube
    edge2.copy(&edge1);
    trans2.translate(side * 4.0/5.0, 0.0, 0.0);
    edge2.matrix_transform(&trans2);

    // Connecting lines
    for i in 0..edge1.col_num {
        let pt1 = &edge1.data[i as usize];
        let pt2 = &edge2.data[i as usize];
        edge3.add_edge(pt1[0], pt1[1], pt1[2], pt2[0], pt2[1], pt2[2]);
    }

    // Drawing and saving
    edge1.draw_lines_xy(&mut img, constant::WHITE_PIXEL);
    edge2.draw_lines_xy(&mut img, constant::WHITE_PIXEL);
    edge3.draw_lines_xy(&mut img, constant::WHITE_PIXEL);
    file::create_ppm_ascii("image/ppm/w03_tesseract.ppm", &img);
}

/// Function that creates all images from work 03
pub fn create_work03_images(mode : i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating test image
    parse::parse("data/w03/w03_script", 500);

    // Creating tesseract image
    tesseract(250.0);
    if (mode == 0) {file::open_image("image/ppm/w03_tesseract.ppm");}

    // Creating impossible triangle
    parse::parse("data/w03/w03_triangle", 750);

    // Creating dragon image
    parse::parse("data/w03/w03_dragon", 750);
}
