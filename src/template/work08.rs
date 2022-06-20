// Imports
use crate::template::prev::ver08::algorithm::{line, shape};
use crate::template::prev::ver08::format::{constant, file, image::Image, matrix::Matrix, pixel::Pixel};
use crate::template::prev::ver08::script::parse;
use rand::{rngs::ThreadRng, Rng};
use std::{f32::consts::PI, fs, process::Command};

/// Function that adds a star to an image with a given color and size
pub fn add_star(img: &mut Image, p: (i32, i32, f32), sz: i32, pix: Pixel) {
    // Creating gradient function
    let grad = |x: i32, y: i32| -> Pixel {
        let rm: f32 = (255_f32 / pix.get_red() as f32);
        let gm: f32 = (255_f32 / pix.get_green() as f32);
        let bm: f32 = (255_f32 / pix.get_blue() as f32);
        let drm: f32 =
            (1_f32 - ((p.0 - x).abs() + (p.1 - y).abs()) as f32 / sz as f32) * (rm - 1_f32) + rm;
        let dgm: f32 =
            (1_f32 - ((p.0 - x).abs() + (p.1 - y).abs()) as f32 / sz as f32) * (gm - 1_f32) + gm;
        let dbm: f32 =
            (1_f32 - ((p.0 - x).abs() + (p.1 - y).abs()) as f32 / sz as f32) * (bm - 1_f32) + gm;
        Pixel::new_value(
            (drm * pix.get_red() as f32) as u8,
            (dgm * pix.get_green() as f32) as u8,
            (dbm * pix.get_blue() as f32) as u8,
        )
    };

    // Creating distance function
    let dist =
        |cx: i32, cy: i32, x: i32, y: i32| -> i32 { (cx - x) * (cx - x) + (cy - y) * (cy - y) };

    // Adding background gradient
    for x in (p.0 - sz)..=(p.0 + sz) {
        for y in (p.1 - sz)..=(p.1 + sz) {
            // Making sure in bounds
            if (x < 0 || x >= img.get_width() || y < 0 || y >= img.get_height()) {
                continue;
            }

            // Making sure within proper distance
            if ((dist(p.0 - sz, p.1 - sz, x, y) < ((sz * sz) as f32 / 1.5) as i32)
                || (dist(p.0 - sz, p.1 + sz, x, y) < ((sz * sz) as f32 / 1.5) as i32)
                || (dist(p.0 + sz, p.1 + sz, x, y) < ((sz * sz) as f32 / 1.5) as i32)
                || (dist(p.0 + sz, p.1 - sz, x, y) < ((sz * sz) as f32 / 1.5) as i32))
            {
                continue;
            }

            // Updating pixel
            img.update_pixel_xy(x, y, p.2, grad(x, y));
        }
    }
}

/// Function that creates a color wheel image
pub fn color_wheel(mode: i32) {
    // Variable declarations
    let mut img: Image = Image::new_dimension(750, 750);
    let mut poly: Matrix = Matrix::new_matrix();
    let mut cord: Matrix = Matrix::new_transformation();
    let trans: Matrix = Matrix::translate(375.0, 375.0, 0.0);

    // Creating sphere and translating
    cord.right_transform(&trans);
    shape::add_sphere(&mut poly, (0.0, 0.0, 0.0), 300.0, 2000);
    poly.left_transform(&cord);

    // Drawing sphere with gradient
    for i in 0..poly.data.len() {
        if (i % 3 == 2) {
            // Getting angle
            let dx: f32 = poly.data[i][0] - 375.0;
            let dy: f32 = poly.data[i][1] - 375.0;
            let angle: f32 = (dy / dx).atan();

            // Getting pixel value from angle
            let pix: Pixel;
            if (dx < 0.0 && angle <= -PI / 6.0) {
                pix = Pixel::new_value(
                    255,
                    0,
                    (255.0 * (1.0 - ((angle * -1.0) - (PI / 6.0)) / (PI / 3.0))) as u8,
                );
            } else if (dx < 0.0 && angle <= PI / 6.0) {
                pix = Pixel::new_value(
                    (255.0 * (1.0 - (angle + PI / 6.0) / (PI / 3.0))) as u8,
                    0,
                    255,
                );
            } else if (dx < 0.0) {
                pix = Pixel::new_value(0, (255.0 * ((angle - PI / 6.0) / (PI / 3.0))) as u8, 255);
            } else if (dx >= 0.0 && angle <= -PI / 6.0) {
                pix = Pixel::new_value(
                    0,
                    255,
                    (255.0 * (((angle * -1.0) - (PI / 6.0)) / (PI / 3.0))) as u8,
                );
            } else if (dx >= 0.0 && angle <= PI / 6.0) {
                pix = Pixel::new_value((255.0 * ((angle + PI / 6.0) / (PI / 3.0))) as u8, 255, 0);
            } else {
                pix = Pixel::new_value(
                    255,
                    (255.0 * (1.0 - (angle - PI / 6.0) / (PI / 3.0))) as u8,
                    0,
                );
            }

            // Performing scanline
            line::scanline(&poly, i as i32, &mut img, pix)
        }
    }

    // Saving image
    file::create_ppm_ascii("image/ppm/w08_color_wheel.ppm", &img, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w08_color_wheel.ppm");
    }
}

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

/// Function that creates the night sky gif
pub fn night_sky() {
    // Variable declarations
    let mut img: Image = Image::new_dimension(750, 750);
    let mut stars: Vec<(i32, i32, f32, i32, bool, Pixel)> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut rng: ThreadRng = rand::thread_rng();
    let colors: Vec<Pixel> = vec![
        Pixel::new_value(165, 185, 255),
        Pixel::new_value(199, 214, 255),
        Pixel::new_value(241, 239, 255),
        Pixel::new_value(255, 245, 242),
        Pixel::new_value(255, 224, 188),
        Pixel::new_value(255, 223, 181),
        Pixel::new_value(255, 202, 138),
        Pixel::new_value(255, 204, 138),
    ];

    // Attempting to create image directory
    fs::create_dir_all("temp").expect("Unable to create temp directory");
    fs::create_dir_all("image/gif").expect("Unable to create image/gif directory");

    // Iterating through scenes
    for i in 0..180 {
        // Implementing background gradient
        img.gradient(
            |x, y| {
                Pixel::new_value(
                    (19_f32 * (1_f32 - (((y as f32 * 0.95) + (x as f32 * 0.05)) / 750_f32))) as u8,
                    (24_f32 * (1_f32 - (((y as f32 * 0.95) + (x as f32 * 0.05)) / 750_f32))) as u8,
                    (98_f32 * (1_f32 - (((y as f32 * 0.95) + (x as f32 * 0.05)) / 750_f32))) as u8,
                )
            },
            0.0,
        );

        // Adding new stars
        stars.push((
            rng.gen_range(10..img.get_width() - 10),
            rng.gen_range(10..img.get_height() - 10),
            rng.gen_range(-100000_f32..100000_f32),
            0,
            true,
            colors[(i / 5) % colors.len()],
        ));

        // Updating sizes of current stars
        for star in stars.iter_mut() {
            if (star.4 && star.3 >= 8) {
                star.4 = false;
                star.3 -= 1;
            } else if (star.4) {
                star.3 += 1;
            } else if (!star.4 && star.3 == 0) {
                star.4 = true;
                star.3 += 1;
            } else {
                star.3 -= 1;
            }
        }

        // Drawing stars
        for star in stars.iter() {
            add_star(&mut img, (star.0, star.1, star.2), star.3, star.5);
        }

        // Saving image
        let name: String = vec![
            "temp/night_sky".to_string(),
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
    println!("Converting night_sky images to w08_night_sky.gif...");
    Command::new("convert")
        .arg("-delay")
        .arg("1.7")
        .arg("-loop")
        .arg("0")
        .arg("temp/night_sky*.ppm")
        .arg("image/gif/w08_night_sky.gif")
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named image/gif/w08_night_sky.gif");

    // Removing temporary files
    for file in names.iter() {
        fs::remove_file(&file).expect("Unable to delete temporary gif file");
    }
}

/// Function that creates all images from work 08
pub fn create_work08_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating shapes image test
    parse::parse("data/mdl/w08_script.mdl", 750, mode);

    // Creating gradient sphere image
    gradient_sphere(mode);

    // Creating night sky gif
    night_sky();

    // Creatng color wheel iamge
    color_wheel(mode);
}
