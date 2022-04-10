// Imports
use crate::prev::ver06::algorithm::shape;
use crate::prev::ver06::format::{constant, file, image::Image, matrix::Matrix, pixel::Pixel};
use crate::prev::ver06::script::parse;
use crate::template::work00;
use std::fs;

/// Function that makes a filled sphere
pub fn filled_sphere(itr: u32) {
    // Variable declarations
    let mut img: Image = Image::new_dimension(750, 750);
    let mut sphere1: Matrix = shape::gen_sphere((0.0, 0.0, 0.0), 300.0, itr);
    let mut sphere2: Matrix = shape::gen_sphere((0.0, 0.0, 0.0), 300.0, itr);
    let mut edge: Matrix = Matrix::new_matrix();
    let mut trans: Matrix = Matrix::new_transformation();

    // Transforming spheres
    trans.rotate_degree(90.0, "z");
    sphere1.matrix_transform(&trans);
    trans = Matrix::new_transformation();
    trans.translate(375.0, 375.0, 0.0);
    sphere1.matrix_transform(&trans);
    sphere2.matrix_transform(&trans);

    // Adding actual lines to sphere
    for i in 0..sphere1.data.len() {
        if (i % (itr + 1) as usize == 0 || i % (itr + 1) as usize == itr as usize) {
            edge.add_col(&sphere1.data[i]);
        } else {
            edge.add_col(&sphere1.data[i]);
            edge.add_col(&sphere1.data[i]);
        }
    }
    for i in 0..sphere2.data.len() {
        if (i % (itr + 1) as usize == 0 || i % (itr + 1) as usize == itr as usize) {
            edge.add_col(&sphere2.data[i]);
        } else {
            edge.add_col(&sphere2.data[i]);
            edge.add_col(&sphere2.data[i]);
        }
    }

    // Add to image and saving
    edge.draw_lines_xy(&mut img, constant::WHITE_PIXEL);
    file::create_ppm_ascii("image/ppm/w05_filled_sphere.ppm", &img, 0);
}

/// Function that makes an rgb sphere
pub fn rgb_sphere(itr: u32) {
    // Variable declarations
    let mut img: Image = Image::new_dimension(750, 750);
    let mut sphere1: Matrix = shape::gen_sphere((0.0, 0.0, 0.0), 300.0, itr);
    let mut sphere2: Matrix = shape::gen_sphere((0.0, 0.0, 0.0), 300.0, itr);
    let mut edge: Matrix = Matrix::new_matrix();
    let mut trans: Matrix = Matrix::new_transformation();

    // Transforming spheres
    trans.rotate_degree(90.0, "z");
    sphere1.matrix_transform(&trans);
    trans = Matrix::new_transformation();
    trans.translate(375.0, 375.0, 0.0);
    sphere1.matrix_transform(&trans);
    sphere2.matrix_transform(&trans);

    // Adding actual lines to sphere
    for i in 0..sphere1.data.len() {
        if (i % (itr + 1) as usize == 0 || i % (itr + 1) as usize == itr as usize) {
            edge.add_col(&sphere1.data[i]);
        } else {
            edge.add_col(&sphere1.data[i]);
            edge.add_col(&sphere1.data[i]);
        }
    }
    for i in 0..sphere2.data.len() {
        if (i % (itr + 1) as usize == 0 || i % (itr + 1) as usize == itr as usize) {
            edge.add_col(&sphere2.data[i]);
        } else {
            edge.add_col(&sphere2.data[i]);
            edge.add_col(&sphere2.data[i]);
        }
    }

    // Adding lines and color to image
    edge.draw_lines_xy(&mut img, constant::WHITE_PIXEL);
    for i in 0..750 {
        for q in 0..750 {
            if (img.get_pixel_rc(i, q) != constant::BLACK_PIXEL) {
                continue;
            }
            let r: u8 = 0;
            let g: u8 = (work00::number_grid_helper(i, q) % 256) as u8;
            let b: u8 = (work00::number_grid_helper(q / 2, i / 2) % 256) as u8;
            img.flood_rc(i, q, constant::WHITE_PIXEL, Pixel::new_value(r, g, b));
        }
    }

    file::create_ppm_ascii("image/ppm/w05_rgb_sphere.ppm", &img, 0);
}

/// Function that creates all images from work 05
pub fn create_work05_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating shapes image test
    parse::parse("data/w05/w05_script", 750, mode);

    // Creating 3d face image
    parse::parse("data/w05/w05_face", 750, mode);

    // Creating solar system image
    parse::parse("data/w05/w05_solar", 750, mode);

    // Creating filled in sphere image
    filled_sphere(800);
    if (mode == 0) {
        file::open_image("image/ppm/w05_filled_sphere.ppm");
    }

    // Creating rgb sphere image
    rgb_sphere(200);
    if (mode == 0) {
        file::open_image("image/ppm/w05_rgb_sphere.ppm");
    }
}
