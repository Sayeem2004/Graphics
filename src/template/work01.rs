// Imports
use crate::format::{image::Image, pixel::Pixel, constant, file};
use crate::algorithm::{line, transform};
use std::fs;

// Function that tests lines in all octants
pub fn work01_lines(img : &mut Image) {
    // Initializing pixel color
    let mut pix : Pixel = Pixel::new_value(0, 255, 0);

    // Octants 1 and 5
    line::draw_line(0, 0, img.get_width()-1, img.get_height()-1, img, pix);
    line::draw_line(0, 0, img.get_width()-1, img.get_height()/2, img, pix);
    line::draw_line(img.get_width()-1, img.get_height()-1, 0, img.get_height()/2, img, pix);

    // Octants 8 and 4
    pix.change_blue(255);
    line::draw_line(0, img.get_height()-1, img.get_width()-1, 0, img, pix);
    line::draw_line(0, img.get_height()-1, img.get_width()-1, img.get_height()/2, img, pix);
    line::draw_line(img.get_width()-1, 0, 0, img.get_height()/2, img, pix);

    // Octants 2 and 6
    pix.change_red(255);
    pix.change_green(0);
    pix.change_blue(0);
    line::draw_line(0, 0, img.get_width()/2, img.get_height()-1, img, pix);
    line::draw_line(img.get_width()-1, img.get_height()-1, img.get_width()/2, 0, img, pix);

    // Octants 7 and 3
    pix.change_blue(255);
    line::draw_line(0, img.get_height()-1, img.get_width()/2, 0, img, pix);
    line::draw_line(img.get_width()-1, 0, img.get_width()/2, img.get_height()-1, img, pix);

    // Horizontal and vertical
    pix.change_blue(0);
    pix.change_green(255);
    line::draw_line(0, img.get_height()/2, img.get_width()-1, img.get_height()/2, img, pix);
    line::draw_line(img.get_width()/2, 0, img.get_width()/2, img.get_height()-1, img, pix);
}

// Function that draws sierpinski's triangle on an image
pub fn sierpinski(img : &mut Image, pix : Pixel, x0 : u32, y0 : u32, x1 : u32, y1 : u32, n : u32) {
    // Base case
    if (n == 0) {return;}

    // Drawing triangle
    line::draw_line(x0, y0, x1, y1, img, pix);
    line::draw_line(x0, y0, x0+(x1-x0)/2, y0+((((x1-x0)/2) as f64)*f64::sqrt(3.0)) as u32, img, pix);
    line::draw_line(x1, y1, x0+(x1-x0)/2, y0+((((x1-x0)/2) as f64)*f64::sqrt(3.0)) as u32, img, pix);

    // Updating bounds and doing recursion
    sierpinski(img, pix, x0, y0, ((x0+x1)/2) as u32, y1, n-1);
    sierpinski(img, pix, ((x0+x1)/2) as u32, y0, x1, y1, n-1);
    sierpinski(img, pix, x0+(x1-x0)/4, y0+(((((x1-x0)/2) as f64)*f64::sqrt(3.0)) as u32)/2, x1-(x1-x0)/4, y0+(((((x1-x0)/2) as f64)*f64::sqrt(3.0)) as u32)/2, n-1);
}

// Function that draws the heighway dragon on an image
pub fn heighway(img : &mut Image, pix : Pixel, x0 : u32, y0 : u32, x1 : u32, y1 : u32, n : u32, angle : f64, scale : f64) {
    // Base case
    if (n == 0) {
        line::draw_line(x0, y0, x1, y1, img, pix);
        return;
    }

    // Creating line copies
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = (x0, y0, x1, y1);
    let (l5, l6, l7, l8) : (u32, u32, u32, u32) = (x1, y1, x0, y0);

    // Rotating lines
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = transform::rotate_degree(l1, l2, l3, l4, angle);
    let (l5, l6, l7, l8) : (u32, u32, u32, u32) = transform::rotate_degree(l5, l6, l7, l8, -angle);

    // Scaling lines
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = transform::dilate(l1, l2, l3, l4, scale);
    let (l5, l6, l7, l8) : (u32, u32, u32, u32) = transform::dilate(l5, l6, l7, l8, scale);

    // Recursive step
    heighway(img, pix, l1, l2, l3, l4, n-1, angle, scale);
    heighway(img, pix, l5, l6, l7, l8, n-1, angle, scale);
}

// Function that draws a binary tree fractal on an image
pub fn bintree(img : &mut Image, pix : Pixel, x0 : u32, y0 : u32, x1 : u32, y1 : u32, n : u32, angle : f64, scale : f64) {
    // Base case
    if (n == 0) {return;}

    // Drawing line
    line::draw_line(x0, y0, x1, y1, img, pix);

    // Left recursion
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = (x1, y1, x1+x1-x0, y1+y1-y0);
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = transform::rotate_degree(l1, l2, l3, l4, angle);
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = transform::dilate(l1, l2, l3, l4, scale);
    bintree(img, pix, l1, l2, l3, l4, n-1, angle, scale);

    // Right recursion
    let (l5, l6, l7, l8) : (u32, u32, u32, u32) = (x1, y1, x1+x1-x0, y1+y1-y0);
    let (l5, l6, l7, l8) : (u32, u32, u32, u32) = transform::rotate_degree(l5, l6, l7, l8, -angle);
    let (l5, l6, l7, l8) : (u32, u32, u32, u32) = transform::dilate(l5, l6, l7, l8, scale);
    bintree(img, pix, l5, l6, l7, l8, n-1, angle, scale);
}

// Function that draws a reflected koch snowflake on an image
pub fn koch(img : &mut Image, pix : Pixel, x0 : u32, y0 : u32, x1 : u32, y1 : u32, n : u32, angle : f64) {
    // Base case
    if (n == 0) {
        line::draw_line(x0, y0, x1, y1, img, pix);
        return;
    }

    // First straight line recursion
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = (x0, y0, x0+x1/3-x0/3, y0+y1/3-y0/3);
    koch(img, pix, l1, l2, l3, l4, n-1, angle);

    // Outward triangle
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = (x0+x1/3-x0/3, y0+y1/3-y0/3, x1-x1/3+x0/3, y1-y1/3+y0/3);
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = transform::rotate_degree(l1, l2, l3, l4, angle);
    koch(img, pix, l1, l2, l3, l4, n-1, angle);
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = transform::rotate_degree(l3, l4, l1, l2, angle);
    koch(img, pix, l1, l2, l3, l4, n-1, angle);

    // Inward triangle
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = (x0+x1/3-x0/3, y0+y1/3-y0/3, x1-x1/3+x0/3, y1-y1/3+y0/3);
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = transform::rotate_degree(l1, l2, l3, l4, -angle);
    koch(img, pix, l1, l2, l3, l4, n-1, angle);
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = transform::rotate_degree(l3, l4, l1, l2, -angle);
    koch(img, pix, l1, l2, l3, l4, n-1, angle);

    // Second straight line recursion
    let (l1, l2, l3, l4) : (u32, u32, u32, u32) = (x1-x1/3+x0/3, y1-y1/3+y0/3, x1, y1);
    koch(img, pix, l1, l2, l3, l4, n-1, angle);
}

// Function that runs all the above pattern functions
pub fn create_work01_images() {
    // Variable declarations
    let size : u32 = 500;

    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Testing draw line function using given instruction
    let mut curr1 : Image = Image::new_dimension(size, size);
    work01_lines(&mut curr1);
    file::create_ppm_ascii("image/ppm/w01_lines.ppm", curr1);

    // Sierpinski triangle fractal
    let mut curr2 : Image = Image::new_dimension(size, size);
    sierpinski(&mut curr2, constant::WHITE_PIXEL, 0, 0, size-1, 0, 8);
    file::create_ppm_ascii("image/ppm/w01_sierpinski.ppm", curr2);

    // Heighway dragon fractal
    let mut curr3 : Image = Image::new_dimension(size, size);
    heighway(&mut curr3, constant::RED_PIXEL, size/20+size/5, size/2, size-size/5+size/20, size/2, 16, 45.0, 1.0 / f64::sqrt(1.9));
    heighway(&mut curr3, constant::BLUE_PIXEL, size/20+size/5, size/2, size-size/5+size/20, size/2, 16, -45.0, 1.0 / f64::sqrt(1.9));
    file::create_ppm_ascii("image/ppm/w01_heighway.ppm", curr3);

    // Binary tree fractal
    let mut curr4 : Image = Image::new_dimension(size, size);
    bintree(&mut curr4, constant::AQUA_PIXEL, 0, 0, size/8, size/8, 14, 10.0, 0.87);
    file::create_ppm_ascii("image/ppm/w01_bintree.ppm", curr4);

    // Koch snowflake fractal
    let mut curr5 : Image = Image::new_dimension(size, size);
    koch(&mut curr5, constant::PURPLE_PIXEL, size/8, size/3, size-size/8, size/3, 8, 60.0);
    koch(&mut curr5, constant::PURPLE_PIXEL, size/8, size/3, size/2, size/3+(((3*size/8) as f64) * f64::sqrt(3.0)) as u32, 8, 60.0);
    koch(&mut curr5, constant::PURPLE_PIXEL, size-size/8, size/3, size/2, size/3+(((3*size/8) as f64) * f64::sqrt(3.0)) as u32, 8, 60.0);
    file::create_ppm_ascii("image/ppm/w01_koch.ppm", curr5);
}
