// Imports
use crate::format::{file, image::Image, matrix::Matrix, constant};
use crate::algorithm::matrix;
use std::fs;

/// Function thats out the matrix struct and related functions
pub fn test_matrix() {
    // Testing add edge
    println!("Testing adding an edge");
    let mut mat1 = Matrix::new_matrix();
    println!("mat1 = Matrix::new_matrix()");
    print!("mat1 = \n{}", mat1);
    mat1.add_edge(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    println!("mat1.add_edge(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)");
    print!("mat1 = \n{}", mat1);

    // Testing identity matrix
    println!("Testing making an identity matrix");
    let mat2 = Matrix::new_identity();
    println!("mat2 = Matrix::new_identity()");
    print!("mat2 =\n{}", mat2);

    // Testing matrix multiplication
    println!("Testing matrix multiplication");
    let mat3 : Matrix = matrix::multiply_matrices(mat2, mat1);
    println!("mat3 = matrix::multiply_matrices(mat2, mat1)");
    print!("mat3 =\n{}", mat3);

    // Testing matrix multiplication again
    println!("Testing matrix multiplication");
    let mut mat4 : Matrix = Matrix::new_matrix();
    mat4.add_edge(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    mat4.add_edge(7.0, 8.0, 9.0, 10.0, 11.0, 12.0);
    print!("mat4 =\n{}", mat4);
    print!("mat3 =\n{}", mat3);
    let mat5 : Matrix = matrix::multiply_matrices(mat4, mat3);
    println!("mat5 : Matrix = matrix::multiply_matrices(mat4, mat3)");
    print!("mat5 =\n{}", mat5);

    // Testing draw_lines
    println!("Testing drawing all lines");
    let size : i32 = 500;
    let mut img : Image = Image::new_dimension(size, size);
    let mut mat6 : Matrix = Matrix::new_matrix();
    mat6.add_edge(50.0, 450.0, 0.0, 100.0, 450.0, 0.0);
    mat6.add_edge(50.0, 450.0, 0.0, 50.0, 400.0, 0.0);
    mat6.add_edge(100.0, 450.0, 0.0, 100.0, 400.0, 0.0);
    mat6.add_edge(100.0, 400.0, 0.0, 50.0, 400.0, 0.0);

    mat6.add_edge(200.0, 450.0, 0.0, 250.0, 450.0, 0.0);
    mat6.add_edge(200.0, 450.0, 0.0, 200.0, 400.0, 0.0);
    mat6.add_edge(250.0, 450.0, 0.0, 250.0, 400.0, 0.0);
    mat6.add_edge(250.0, 400.0, 0.0, 200.0, 400.0, 0.0);

    mat6.add_edge(150.0, 400.0, 0.0, 130.0, 360.0, 0.0);
    mat6.add_edge(150.0, 400.0, 0.0, 170.0, 360.0, 0.0);
    mat6.add_edge(130.0, 360.0, 0.0, 170.0, 360.0, 0.0);

    mat6.add_edge(100.0, 340.0, 0.0, 200.0, 340.0, 0.0);
    mat6.add_edge(100.0, 320.0, 0.0, 200.0, 320.0, 0.0);
    mat6.add_edge(100.0, 340.0, 0.0, 100.0, 320.0, 0.0);
    mat6.add_edge(200.0, 340.0, 0.0, 200.0, 320.0, 0.0);
    print!("mat6 =\n{}", mat6);
    println!("mat6.draw_lines(img, constant::WHITE_PIXEL)");
    mat6.draw_lines(&mut img, constant::WHITE_PIXEL);
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");
    file::create_ppm_ascii("image/ppm/w02_matrix.ppm", img);
}

/// Function that creates a spiral in an edge matrix based on some parameters
pub fn spiral(mat : &mut Matrix, ds : f32, da : f32, itr : i32, size : i32) {
    let mut prev_size : f32 = 0.0;
    let mut prev_angle : f32 = 0.0;

    for _i in 0..itr.abs() {
        mat.add_edge(
            (size/2) as f32 + f32::cos(prev_angle) * prev_size,
            (size/2) as f32 + f32::sin(prev_angle) * prev_size,
            0.0,
            (size/2) as f32 + f32::cos(prev_angle+da) * (prev_size+ds),
            (size/2) as f32 + f32::sin(prev_angle+da) * (prev_size+ds),
            0.0
        );
        prev_size += ds;
        prev_angle += da;
    }
}

/// Function that creates all images from work 02
pub fn create_work02_images() {
    // Variable declarations
    let size : i32 = 750;

    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Testing matrix functions
    test_matrix();

    // Creating lotus picture
    let mut curr : Image = Image::new_dimension(size, size);
    let mut mat : Matrix = Matrix::new_matrix();
    spiral(&mut mat, 1.0, 1.0, 360, size);
    mat.draw_lines(&mut curr, constant::BLUE_PIXEL);
    mat.clear();
    spiral(&mut mat, 1.0, -1.0, 360, size);
    mat.draw_lines(&mut curr, constant::RED_PIXEL);
    file::create_ppm_ascii("image/ppm/w02_lotus.ppm", curr);
}
