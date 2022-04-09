// Imports
use crate::format::{constant, file, image::Image, matrix::Matrix};
use std::fs;

/// Function thats out the matrix struct and related functions
fn test_matrix(mode: i32) {
    // Testing add edge
    println!("Testing adding an edge");
    let mut mat1: Matrix = Matrix::new_matrix();
    println!("mat1 = Matrix::new_matrix()");
    print!("mat1 = \n{}", mat1);
    mat1.add_edge((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
    println!("mat1.add_edge(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)");
    print!("mat1 = \n{}", mat1);

    // Testing identity matrix
    println!("Testing making an identity matrix");
    let mat2: Matrix = Matrix::new_transformation();
    println!("mat2 = Matrix::new_transformation()");
    print!("mat2 =\n{}", mat2);

    // Testing matrix multiplication
    println!("Testing matrix multiplication");
    let mat3: Matrix = Matrix::multiply_matrices(&mat2, &mat1);
    println!("mat3 = Matrix::multiply_matrices(mat2, mat1)");
    print!("mat3 =\n{}", mat3);

    // Testing matrix multiplication again
    println!("Testing matrix multiplication");
    let mut mat4: Matrix = Matrix::new_matrix();
    mat4.add_edge((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
    mat4.add_edge((7.0, 8.0, 9.0), (10.0, 11.0, 12.0));
    print!("mat4 =\n{}", mat4);
    print!("mat3 =\n{}", mat3);
    let mat5: Matrix = Matrix::multiply_matrices(&mat4, &mat3);
    println!("mat5 : Matrix = Matrix::multiply_matrices(mat4, mat3)");
    print!("mat5 =\n{}", mat5);

    // Testing draw_lines
    println!("Testing drawing all lines");
    let size: i32 = 750;
    let mut img: Image = Image::new_dimension(size, size);
    let mut mat6: Matrix = Matrix::new_matrix();
    mat6.add_edge((50.0, 450.0, 0.0), (100.0, 450.0, 0.0));
    mat6.add_edge((50.0, 450.0, 0.0), (50.0, 400.0, 0.0));
    mat6.add_edge((100.0, 450.0, 0.0), (100.0, 400.0, 0.0));
    mat6.add_edge((100.0, 400.0, 0.0), (50.0, 400.0, 0.0));

    mat6.add_edge((200.0, 450.0, 0.0), (250.0, 450.0, 0.0));
    mat6.add_edge((200.0, 450.0, 0.0), (200.0, 400.0, 0.0));
    mat6.add_edge((250.0, 450.0, 0.0), (250.0, 400.0, 0.0));
    mat6.add_edge((250.0, 400.0, 0.0), (200.0, 400.0, 0.0));

    mat6.add_edge((150.0, 400.0, 0.0), (130.0, 360.0, 0.0));
    mat6.add_edge((150.0, 400.0, 0.0), (170.0, 360.0, 0.0));
    mat6.add_edge((130.0, 360.0, 0.0), (170.0, 360.0, 0.0));

    mat6.add_edge((100.0, 340.0, 0.0), (200.0, 340.0, 0.0));
    mat6.add_edge((100.0, 320.0, 0.0), (200.0, 320.0, 0.0));
    mat6.add_edge((100.0, 340.0, 0.0), (100.0, 320.0, 0.0));
    mat6.add_edge((200.0, 340.0, 0.0), (200.0, 320.0, 0.0));
    print!("mat6 =\n{}", mat6);
    println!("mat6.draw_lines(img, constant::WHITE_PIXEL)");
    mat6.draw_lines_xy(&mut img, constant::WHITE_PIXEL);
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");
    file::create_ppm_ascii("image/ppm/w02_matrix.ppm", &img, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w02_matrix.ppm");
    }
}

/// Function that creates a spiral in an edge matrix based on some parameters
fn spiral(mat: &mut Matrix, ds: f32, da: f32, itr: u32, size: i32) {
    // Error checking
    if (size < 0) {
        eprintln!("Size of spiral can not be negative, no changes made");
        return;
    }

    // Variable declarations
    let mut prev_size: f32 = 0.0;
    let mut prev_angle: f32 = 0.0;

    // Iterating and adding edges
    for _i in 0..itr {
        mat.add_edge(
            (
                (size / 2) as f32 + f32::cos(prev_angle) * prev_size,
                (size / 2) as f32 + f32::sin(prev_angle) * prev_size,
                0.0,
            ),
            (
                (size / 2) as f32 + f32::cos(prev_angle + da) * (prev_size + ds),
                (size / 2) as f32 + f32::sin(prev_angle + da) * (prev_size + ds),
                0.0,
            ),
        );
        prev_size += ds;
        prev_angle += da;
    }
}

/// Function that creates all images from work 02
pub fn create_work02_images(mode: i32) {
    // Variable declarations
    let size: i32 = 750;

    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Testing matrix functions
    test_matrix(mode);

    // Creating lotus picture
    let mut curr1: Image = Image::new_dimension(size, size);
    let mut mat: Matrix = Matrix::new_matrix();
    spiral(&mut mat, 1.0, 1.0, 360, size);
    mat.draw_lines_xy(&mut curr1, constant::BLUE_PIXEL);
    mat.clear();
    spiral(&mut mat, 1.0, -1.0, 360, size);
    mat.draw_lines_xy(&mut curr1, constant::RED_PIXEL);
    file::create_ppm_ascii("image/ppm/w02_lotus.ppm", &curr1, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w02_lotus.ppm");
    }

    // Creating rainbow lotus image
    let mut mat: Matrix = file::read_lines_csv("data/w02/w02_rainbow_lotus.csv");
    let mut trans: Matrix = Matrix::new_transformation();
    trans.dilate(0.8, 0.8, 1.0);
    trans.translate(-7.0, 50.0, 0.0);
    mat.matrix_transform(&trans);
    mat.add_edge((410.0, 215.0, 0.0), (415.0, 215.0, 0.0));
    mat.add_edge((521.0, 275.0, 0.0), (521.0, 270.0, 0.0));
    mat.add_edge((101.0, 380.0, 0.0), (101.0, 372.0, 0.0));
    mat.add_edge((733.0, 430.0, 0.0), (733.0, 434.0, 0.0));
    mat.add_edge((650.0, 380.0, 0.0), (645.0, 380.0, 0.0));
    mat.add_edge((576.0, 240.0, 0.0), (578.0, 240.0, 0.0));
    let mut curr2: Image = Image::new_dimension(size, size);
    mat.draw_lines_rc(&mut curr2, constant::WHITE_PIXEL);
    curr2.flood_xy(375, 425, constant::WHITE_PIXEL, constant::YELLOW_PIXEL);
    curr2.flood_xy(375, 500, constant::WHITE_PIXEL, constant::AQUA_PIXEL);
    curr2.flood_xy(375, 375, constant::WHITE_PIXEL, constant::AQUA_PIXEL);
    curr2.flood_xy(375, 475, constant::WHITE_PIXEL, constant::BLUE_PIXEL);
    curr2.flood_xy(200, 500, constant::WHITE_PIXEL, constant::FUCHSIA_PIXEL);
    curr2.flood_xy(550, 500, constant::WHITE_PIXEL, constant::GREEN_PIXEL);
    curr2.flood_xy(175, 400, constant::WHITE_PIXEL, constant::LIME_PIXEL);
    curr2.flood_xy(150, 400, constant::WHITE_PIXEL, constant::LIME_PIXEL);
    curr2.flood_xy(100, 450, constant::WHITE_PIXEL, constant::MAROON_PIXEL);
    curr2.flood_xy(91, 400, constant::WHITE_PIXEL, constant::MAROON_PIXEL);
    curr2.flood_xy(110, 350, constant::WHITE_PIXEL, constant::NAVY_PIXEL);
    curr2.flood_xy(110, 300, constant::WHITE_PIXEL, constant::NAVY_PIXEL);
    curr2.flood_xy(275, 225, constant::WHITE_PIXEL, constant::OLIVE_PIXEL);
    curr2.flood_xy(475, 225, constant::WHITE_PIXEL, constant::OLIVE_PIXEL);
    curr2.flood_xy(475, 275, constant::WHITE_PIXEL, constant::PURPLE_PIXEL);
    curr2.flood_xy(275, 275, constant::WHITE_PIXEL, constant::PURPLE_PIXEL);
    curr2.flood_xy(250, 300, constant::WHITE_PIXEL, constant::PURPLE_PIXEL);
    curr2.flood_xy(600, 425, constant::WHITE_PIXEL, constant::RED_PIXEL);
    curr2.flood_xy(575, 425, constant::WHITE_PIXEL, constant::RED_PIXEL);
    curr2.flood_xy(650, 425, constant::WHITE_PIXEL, constant::SILVER_PIXEL);
    curr2.flood_xy(675, 310, constant::WHITE_PIXEL, constant::TEAL_PIXEL);
    curr2.flood_xy(675, 325, constant::WHITE_PIXEL, constant::TEAL_PIXEL);
    file::create_ppm_ascii("image/ppm/w02_rainbow_lotus.ppm", &curr2, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w02_rainbow_lotus.ppm");
    }

    // Creating eru image
    let mut mat2: Matrix = file::read_lines_csv("data/w02/w02_eru.csv");
    let mut trans2: Matrix = Matrix::new_transformation();
    trans2.dilate(0.7, 0.85, 1.0);
    trans2.translate(10.0, 10.0, 0.0);
    mat2.matrix_transform(&trans2);
    let mut curr3: Image = Image::new_dimension(size, size);
    mat2.draw_lines_rc(&mut curr3, constant::WHITE_PIXEL);
    file::create_ppm_ascii("image/ppm/w02_eru.ppm", &curr3, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w02_eru.ppm");
    }
}
