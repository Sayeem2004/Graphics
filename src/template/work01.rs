// Imports
use crate::prev::ver05::algorithm::line;
use crate::prev::ver05::format::{constant, file, image::Image, pixel::Pixel};
use std::fs;

/// Function that tests lines in all octants
fn test_lines(img: &mut Image) {
    // Initializing pixel color
    let mut pix: Pixel = Pixel::new_value(0, 255, 0);

    // // Octants 1 and 5
    line::draw_line(0, 0, img.get_width() - 1, img.get_height() - 1, img, pix);
    line::draw_line(0, 0, img.get_width() - 1, img.get_height() / 2, img, pix);
    line::draw_line(
        img.get_width() - 1,
        img.get_height() - 1,
        0,
        img.get_height() / 2,
        img,
        pix,
    );

    // Octants 8 and 4
    pix.change_blue(255);
    line::draw_line(0, img.get_height() - 1, img.get_width() - 1, 0, img, pix);
    line::draw_line(
        0,
        img.get_height() - 1,
        img.get_width() - 1,
        img.get_height() / 2,
        img,
        pix,
    );
    line::draw_line(img.get_width() - 1, 0, 0, img.get_height() / 2, img, pix);

    // Octants 2 and 6
    pix.change_red(255);
    pix.change_green(0);
    pix.change_blue(0);
    line::draw_line(0, 0, img.get_width() / 2, img.get_height() - 1, img, pix);
    line::draw_line(
        img.get_width() - 1,
        img.get_height() - 1,
        img.get_width() / 2,
        0,
        img,
        pix,
    );

    // Octants 7 and 3
    pix.change_blue(255);
    line::draw_line(0, img.get_height() - 1, img.get_width() / 2, 0, img, pix);
    line::draw_line(
        img.get_width() - 1,
        0,
        img.get_width() / 2,
        img.get_height() - 1,
        img,
        pix,
    );

    // Horizontal and vertical
    pix.change_blue(0);
    pix.change_green(255);
    line::draw_line(
        0,
        img.get_height() / 2,
        img.get_width() - 1,
        img.get_height() / 2,
        img,
        pix,
    );
    line::draw_line(
        img.get_width() / 2,
        0,
        img.get_width() / 2,
        img.get_height() - 1,
        img,
        pix,
    );
}

/// Function that draws sierpinski's triangle on an image
fn sierpinski(img: &mut Image, pix: Pixel, s: (i32, i32), e: (i32, i32), n: i32) {
    // Error checking
    if (n < 0) {
        eprintln!("Recursive iteration for sierpinski can not be negative, no changes made");
        return;
    }

    // Base case
    if (n == 0) {
        return;
    }

    // Drawing triangle
    line::draw_line(s.0, s.1, e.0, e.1, img, pix);
    line::draw_line(
        s.0,
        s.1,
        s.0 + (e.0 - s.0) / 2,
        s.1 + ((((e.0 - s.0) / 2) as f32) * f32::sqrt(3.0)) as i32,
        img,
        pix,
    );
    line::draw_line(
        e.0,
        e.1,
        s.0 + (e.0 - s.0) / 2,
        s.1 + ((((e.0 - s.0) / 2) as f32) * f32::sqrt(3.0)) as i32,
        img,
        pix,
    );

    // Updating bounds and doing recursion
    sierpinski(img, pix, s, (((s.0 + e.0) / 2) as i32, e.1), n - 1);
    sierpinski(img, pix, (((s.0 + e.0) / 2) as i32, s.1), e, n - 1);
    sierpinski(
        img,
        pix,
        (
            s.0 + (e.0 - s.0) / 4,
            s.1 + (((((e.0 - s.0) / 2) as f32) * f32::sqrt(3.0)) as i32) / 2,
        ),
        (
            e.0 - (e.0 - s.0) / 4,
            s.1 + (((((e.0 - s.0) / 2) as f32) * f32::sqrt(3.0)) as i32) / 2,
        ),
        n - 1,
    );
}

/// Function that draws the heighway dragon on an image
fn heighway(
    img: &mut Image,
    pix: Pixel,
    s: (i32, i32),
    e: (i32, i32),
    n: i32,
    angle: f32,
    scale: f32,
) {
    // Error checking
    if (n < 0) {
        eprintln!("Recursive iteration for heighway can not be negative, no changes made");
        return;
    }
    if (scale < 0.0) {
        eprintln!("Scale factor for heighway can not be negative, no changes made");
        return;
    }

    // Base case
    if (n == 0) {
        line::draw_line(s.0, s.1, e.0, e.1, img, pix);
        return;
    }

    // Creating line copies
    let mut l1: (i32, i32, i32, i32) = (s.0, s.1, e.0, e.1);
    let mut l2: (i32, i32, i32, i32) = (e.0, e.1, s.0, s.1);

    // Rotating lines
    l1 = line::rotate_degree((l1.0, l1.1), (l1.2, l1.3), angle);
    l2 = line::rotate_degree((l2.0, l2.1), (l2.2, l2.3), -angle);

    // Scaling lines
    l1 = line::dilate((l1.0, l1.1), (l1.2, l1.3), scale);
    l2 = line::dilate((l2.0, l2.1), (l2.2, l2.3), scale);

    // Recursive step
    heighway(img, pix, (l1.0, l1.1), (l1.2, l1.3), n - 1, angle, scale);
    heighway(img, pix, (l2.0, l2.1), (l2.2, l2.3), n - 1, angle, scale);
}

/// Function that draws a binary tree fractal on an image
fn bintree(
    img: &mut Image,
    pix: Pixel,
    s: (i32, i32),
    e: (i32, i32),
    n: i32,
    angle: f32,
    scale: f32,
) {
    // Error checking
    if (n < 0) {
        eprintln!("Recursive iteration for bintree can not be negative, no changes made");
        return;
    }
    if (scale < 0.0) {
        eprintln!("Scale factor for bintree can not be negative, no changes made");
        return;
    }

    // Base case
    if (n == 0) {
        return;
    }

    // Drawing line
    line::draw_line(s.0, s.1, e.0, e.1, img, pix);

    // Left recursion
    let mut l1: (i32, i32, i32, i32) = (e.0, e.1, e.0 + (e.0 - s.0), e.1 + (e.1 - s.1));
    l1 = line::rotate_degree((l1.0, l1.1), (l1.2, l1.3), angle);
    l1 = line::dilate((l1.0, l1.1), (l1.2, l1.3), scale);
    bintree(img, pix, (l1.0, l1.1), (l1.2, l1.3), n - 1, angle, scale);

    // Right recursion
    let mut l2: (i32, i32, i32, i32) = (e.0, e.1, e.0 + (e.0 - s.0), e.1 + (e.1 - s.1));
    l2 = line::rotate_degree((l2.0, l2.1), (l2.2, l2.3), -angle);
    l2 = line::dilate((l2.0, l2.1), (l2.2, l2.3), scale);
    bintree(img, pix, (l2.0, l2.1), (l2.2, l2.3), n - 1, angle, scale);
}

/// Function that draws a reflected koch snowflake on an image
fn koch(img: &mut Image, pix: Pixel, s: (i32, i32), e: (i32, i32), n: i32, angle: f32) {
    // Error checking
    if (n < 0) {
        eprintln!("Recursive iteration for koch can not be negative, no changes made");
        return;
    }

    // Base case
    if (n == 0) {
        line::draw_line(s.0, s.1, e.0, e.1, img, pix);
        return;
    }

    // First straight line recursion
    let l1: (i32, i32, i32, i32) = (s.0, s.1, s.0 + e.0 / 3 - s.0 / 3, s.1 + e.1 / 3 - s.1 / 3);
    koch(img, pix, (l1.0, l1.1), (l1.2, l1.3), n - 1, angle);

    // Outward triangle
    let mut l2: (i32, i32, i32, i32) = (
        s.0 + e.0 / 3 - s.0 / 3,
        s.1 + e.1 / 3 - s.1 / 3,
        e.0 - e.0 / 3 + s.0 / 3,
        e.1 - e.1 / 3 + s.1 / 3,
    );
    l2 = line::rotate_degree((l2.0, l2.1), (l2.2, l2.3), angle);
    koch(img, pix, (l2.0, l2.1), (l2.2, l2.3), n - 1, angle);
    l2 = line::rotate_degree((l2.2, l2.3), (l2.0, l2.1), angle);
    koch(img, pix, (l2.0, l2.1), (l2.2, l2.3), n - 1, angle);

    // Inward triangle
    let mut l3: (i32, i32, i32, i32) = (
        s.0 + e.0 / 3 - s.0 / 3,
        s.1 + e.1 / 3 - s.1 / 3,
        e.0 - e.0 / 3 + s.0 / 3,
        e.1 - e.1 / 3 + s.1 / 3,
    );
    l3 = line::rotate_degree((l3.0, l3.1), (l3.2, l3.3), -angle);
    koch(img, pix, (l3.0, l3.1), (l3.2, l3.3), n - 1, angle);
    l3 = line::rotate_degree((l3.2, l3.3), (l3.0, l3.1), -angle);
    koch(img, pix, (l3.0, l3.1), (l3.2, l3.3), n - 1, angle);

    // Second straight line recursion
    let l4: (i32, i32, i32, i32) = (e.0 - e.0 / 3 + s.0 / 3, e.1 - e.1 / 3 + s.1 / 3, e.0, e.1);
    koch(img, pix, (l4.0, l4.1), (l4.2, l4.3), n - 1, angle);
}

/// Function that runs all the above pattern functions
pub fn create_work01_images(mode: i32) {
    // Variable declarations
    let size: i32 = 750;

    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Testing draw line function using given instruction
    let mut curr1: Image = Image::new_dimension(size, size);
    test_lines(&mut curr1);
    file::create_ppm_ascii("image/ppm/w01_lines.ppm", &curr1, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w01_lines.ppm");
    }

    // Sierpinski triangle fractal
    let mut curr2: Image = Image::new_dimension(size, size);
    sierpinski(&mut curr2, constant::WHITE_PIXEL, (0, 0), (size - 1, 0), 8);
    file::create_ppm_ascii("image/ppm/w01_sierpinski.ppm", &curr2, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w01_sierpinski.ppm");
    }

    // Heighway dragon fractal
    let mut curr3: Image = Image::new_dimension(size, size);
    heighway(
        &mut curr3,
        constant::RED_PIXEL,
        (size / 20 + size / 5, size / 2),
        (size - size / 5 + size / 20, size / 2),
        16,
        45.0,
        1.0 / f32::sqrt(1.9),
    );
    heighway(
        &mut curr3,
        constant::BLUE_PIXEL,
        (size / 20 + size / 5, size / 2),
        (size - size / 5 + size / 20, size / 2),
        16,
        -45.0,
        1.0 / f32::sqrt(1.9),
    );
    file::create_ppm_ascii("image/ppm/w01_heighway.ppm", &curr3, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w01_heighway.ppm");
    }

    // Binary tree fractal
    let mut curr4: Image = Image::new_dimension(size, size);
    bintree(
        &mut curr4,
        constant::AQUA_PIXEL,
        (0, 0),
        (size / 8, size / 8),
        14,
        10.0,
        0.85,
    );
    file::create_ppm_ascii("image/ppm/w01_bintree.ppm", &curr4, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w01_bintree.ppm");
    }

    // Koch snowflake fractal
    let mut curr5: Image = Image::new_dimension(size, size);
    koch(
        &mut curr5,
        constant::PURPLE_PIXEL,
        (size / 8, size / 3),
        (size - size / 8, size / 3),
        8,
        60.0,
    );
    koch(
        &mut curr5,
        constant::PURPLE_PIXEL,
        (size / 8, size / 3),
        (
            size / 2,
            size / 3 + (((3 * size / 8) as f32) * f32::sqrt(3.0)) as i32,
        ),
        8,
        60.0,
    );
    koch(
        &mut curr5,
        constant::PURPLE_PIXEL,
        (size - size / 8, size / 3),
        (
            size / 2,
            size / 3 + (((3 * size / 8) as f32) * f32::sqrt(3.0)) as i32,
        ),
        8,
        60.0,
    );
    file::create_ppm_ascii("image/ppm/w01_koch.ppm", &curr5, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w01_koch.ppm");
    }
}
