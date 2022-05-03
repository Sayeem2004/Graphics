// Imports
use crate::algorithm::shape;
use crate::format::{constant, file, image::Image, matrix::Matrix, pixel::Pixel};
use crate::script::parse;
use std::{fs, process::Command};

/// Function that creates the rotating slab gif
pub fn rotating_slab() {
    // Variable declarations
    let mut img: Image = Image::new_dimension(500, 500);
    let mut names: Vec<String> = Vec::new();

    // Attempting to create image directory
    fs::create_dir_all("image/tmp").expect("Unable to create image/tmp directory");
    fs::create_dir_all("image/gif").expect("Unable to create image/gif directory");

    // Iterating through scenes
    for i in 0..180 {
        // Iterating through rectangles
        let mut x: f32 = -225.0;
        while (x <= 200.0) {
            let mut y: f32 = 50.0;
            while (y <= 475.0) {
                // Uodating coordinate system
                let mut cord: Matrix = Matrix::new_transformation();
                let trans1: Matrix = Matrix::rotate_degree(2.0 * i as f32, "y");
                let trans2: Matrix = Matrix::translate(25.0, 0.0, 0.0);
                cord.right_transform(&trans2);
                cord.right_transform(&trans1);

                // Adding box
                let mut poly: Matrix = Matrix::new_matrix();
                shape::add_box(&mut poly, (x, y, 0.0), 25.0, 25.0, 25.0);
                poly.left_transform(&cord);
                poly.draw_triangles_xy(
                    &mut img,
                    Pixel::new_scale(0.25),
                    (Pixel::new_scale(1.0), 375.0, 375.0, 400.0),
                    constant::ZVIEW,
                    constant::EQV,
                );

                // Updating iterator
                y += 12.5;
            }

            // Updating itrator
            x += 12.5;
        }

        // Saving image
        let name = vec![
            "image/tmp/rotating_slab".to_string(),
            format!("{:0>#3}", i.to_string()),
            ".ppm".to_string(),
        ]
        .join("");
        file::create_ppm_ascii(&name, &img, 0);
        names.push(name);

        // Clearing image
        img.reset_buff();
        img.fill(constant::BLACK_PIXEL);
    }

    // Performing image magick convert command
    println!("Converting night_sky images to w09_rotating_slab.gif...");
    Command::new("convert")
        .arg("-delay")
        .arg("5")
        .arg("-loop")
        .arg("0")
        .arg("image/tmp/rotating_slab*.ppm")
        .arg("image/gif/w09_rotating_slab.gif")
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named image/gif/w09_rotating_slab.gif");

    // Removing temporary files
    for file in names.iter() {
        fs::remove_file(&file).expect("Unable to delete temporary gif file");
    }
}

/// Function that creates all images from work 09
pub fn create_work09_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating face image test
    parse::parse("data/w09/w09_face", 500, mode);

    // Creating robot image test
    parse::parse("data/w09/w09_robot", 500, mode);

    // Creating shapes image test
    parse::parse("data/w09/w09_shapes", 500, mode);

    // Creating rotating slab gif
    rotating_slab();
}
