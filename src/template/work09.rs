// Imports
use crate::prev::ver09::algorithm::shape;
use crate::prev::ver09::format::{constant, file, image::Image, matrix::Matrix, pixel::Pixel};
use crate::prev::ver09::script::parse;
use std::{fs, process::Command};

/// Function that creates the rotating slab gif
pub fn rotating_slab() {
    // Variable declarations
    let sz: i32 = 750;
    let mut img: Image = Image::new_dimension(sz, sz);
    let mut names: Vec<String> = Vec::new();

    // Attempting to create image directory
    fs::create_dir_all("temp").expect("Unable to create temp directory");
    fs::create_dir_all("image/gif").expect("Unable to create image/gif directory");

    // Iterating through scenes
    for i in 0..180 {
        // Iterating through rectangles
        let mut x: f32 = (-sz / 2 + 25) as f32;
        while (x <= (sz / 2) as f32 - 37.5) {
            let mut y: f32 = 37.5;
            while (y <= (sz - 25) as f32) {
                // Updating coordinate system
                let mut cord: Matrix = Matrix::new_transformation();
                let trans1: Matrix = Matrix::rotate_degree(2.0 * i as f32, "y");
                let trans2: Matrix = Matrix::translate((sz / 2) as f32, 0.0, 0.0);
                cord.right_transform(&trans2);
                cord.right_transform(&trans1);

                // Adding box
                let mut poly: Matrix = Matrix::new_matrix();
                shape::add_box(&mut poly, (x, y, 0.0), 12.5, 12.5, 12.5);
                poly.left_transform(&cord);
                poly.draw_triangles_xy(
                    &mut img,
                    Pixel::new_scale(0.25),
                    &[(Pixel::new_scale(1.0), 375.0, 375.0, 400.0)],
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
            "temp/rotating_slab".to_string(),
            format!("{:0>#3}", i),
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
        .arg("temp/rotating_slab*.ppm")
        .arg("image/gif/w09_rotating_slab.gif")
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named image/gif/w09_rotating_slab.gif");

    // Removing temporary files
    for file in names.iter() {
        fs::remove_file(&file).expect("Unable to delete temporary gif file");
    }
}

/// Function that creates the colorful sphere image
pub fn colorful_marble(mode: i32) {
    // Variable declarations
    let itr: i32 = 500;
    let sz: i32 = 750;
    let mut img: Image = Image::new_dimension(sz, sz);
    let mut poly: Matrix = Matrix::new_matrix();
    shape::add_sphere(
        &mut poly,
        ((sz / 2) as f32, (sz / 2) as f32, (sz / 2) as f32),
        sz as f32 / 3.0,
        itr as u32,
    );

    // Creating point light sources
    let pnts: Vec<(Pixel, f32, f32, f32)> = vec![
        (Pixel::new_value(255, 255, 255), 0_f32, 0_f32, 1000_f32),
        (
            Pixel::new_value(255, 255, 0),
            0_f32,
            (sz / 2) as f32,
            1000_f32,
        ),
        (Pixel::new_value(255, 0, 255), 0_f32, sz as f32, 1000_f32),
        (
            Pixel::new_value(0, 255, 255),
            (sz / 2) as f32,
            0_f32,
            1000_f32,
        ),
        (Pixel::new_value(0, 0, 255), sz as f32, 0_f32, 1000_f32),
        (
            Pixel::new_value(0, 255, 0),
            sz as f32,
            (sz / 2) as f32,
            1000_f32,
        ),
        (
            Pixel::new_value(255, 0, 0),
            (sz / 2) as f32,
            sz as f32,
            1000_f32,
        ),
        (
            Pixel::new_value(255, 255, 255),
            (sz + 1) as f32,
            (sz + 1) as f32,
            1000_f32,
        ),
    ];

    // Modifying image
    poly.draw_triangles_xy(
        &mut img,
        Pixel::new_value(0, 0, 192),
        &pnts,
        constant::ZVIEW,
        constant::EQV,
    );

    // Saving image and displaying
    file::create_ppm_ascii("image/ppm/w09_marble.ppm", &img, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w09_marble.ppm");
    }
}

/// Function that creates the bright sphere image
pub fn bright_sphere(mode: i32) {
    // Variable declarations
    let itr: u32 = 100;
    let sz: i32 = 750;
    let mut img: Image = Image::new_dimension(sz, sz);

    // Creating point light sources
    let pnts: Vec<(Pixel, f32, f32, f32)> = vec![(
        Pixel::new_value(200, 200, 224),
        (sz / 2) as f32,
        (sz / 2) as f32,
        0.0,
    )];

    // Adding rings around light
    for i in 0..36 {
        // Creating transformations and torus
        let mut poly: Matrix = Matrix::new_matrix();
        shape::add_torus(&mut poly, (0.0, 0.0, 0.0), 5.0, (sz / 3) as f32, itr);
        let rotx: Matrix = Matrix::rotate_degree((i * 10) as f32, "x");
        let roty: Matrix = Matrix::rotate_degree(-30_f32, "y");
        let rotz: Matrix = Matrix::rotate_degree(60_f32, "z");
        let trans: Matrix = Matrix::translate((sz / 2) as f32, (sz / 2) as f32, 0.0);

        // Transformations
        poly.left_transform(&rotx);
        poly.left_transform(&roty);
        poly.left_transform(&rotz);
        poly.left_transform(&trans);

        // Modifying image
        poly.draw_triangles_xy(
            &mut img,
            Pixel::new_value(200, 200, 255),
            &pnts,
            constant::ZVIEW,
            constant::EQV,
        );
    }

    // Saving image and displaying
    file::create_ppm_ascii("image/ppm/w09_bright_sphere.ppm", &img, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w09_bright_sphere.ppm");
    }
}

/// Function that creates all images from work 09
pub fn create_work09_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating face image test
    parse::parse("data/w09/w09_face", 750, mode);

    // Creating robot image test
    parse::parse("data/w09/w09_robot", 750, mode);

    // Creating shapes image test
    parse::parse("data/w09/w09_shapes", 750, mode);

    // Creating rotating slab gif
    rotating_slab();

    // Creating colorful marble image
    colorful_marble(mode);

    // Creating dyson sphere iamge
    bright_sphere(mode);
}
