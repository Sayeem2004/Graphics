// Imports
use crate::format::{image, file};
use std::fs;

#[allow(dead_code)]
// Converts an image into the number spiral gradient
pub fn number_spiral(img : &mut image::Image, scale1: f32, scale2: f32) {
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            let r : u32 = 0;
            let g : u32 = number_spiral_helper(((i as f32) * scale1) as u32, ((q as f32) * scale1) as u32) % 256;
            let b : u32 = number_spiral_helper(((q as f32) * scale2) as u32, ((i as f32) * scale2) as u32) % 256;
            img.update_pixel(i, q, r, g, b);
        }
    }
}

#[allow(dead_code)]
// Gradient function based on CSES number spiral problem
fn number_spiral_helper(row : u32, col : u32) -> u32 {
    let num : u32;
    if (row > col) {
        if (row % 2 == 0) {num = row * row - col + 1;}
        else {num = (row - 1) * (row - 1) + col;}
    } else if (col > row) {
        if (col % 2 == 1) {num = col * col - row + 1;}
        else {num = (col - 1) * (col - 1) + row;}
    } else {
        if (row % 2 == 0) {num = row * row - col + 1;}
        else {num = col * col - row + 1;}
    }
    return num;
}

#[allow(dead_code)]
// Converts an image into the number grid gradient
pub fn number_grid(img : &mut image::Image, scale1 : f32, scale2 : f32) {
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            let r : u32 = 0;
            let g : u32 = number_grid_helper(((i as f32) * scale1) as u32, ((q as f32) * scale1) as u32) % 256;
            let b : u32 = number_grid_helper(((q as f32) * scale2) as u32, ((i as f32) * scale2) as u32) % 256;
            img.update_pixel(i, q, r, g, b);
        }
    }
}

#[allow(dead_code)]
// Gradient function based on CSES number grid problem
fn number_grid_helper(row : u32, col : u32) -> u32 {
    return (row-1)^(col-1);
}

#[allow(dead_code)]
// Converts an image into a counting bits gradient
pub fn counting_bits(img : &mut image::Image, scale1 : f32, scale2 : f32) {
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            let r : u32 = 0;
            let g : u32 = (counting_bits_helper((((i^q) as f32) * scale1) as u64) % 256) as u32;
            let b : u32 = (counting_bits_helper((((i^q) as f32) * scale2) as u64) % 256) as u32;
            img.update_pixel(i, q, r, g, b);
        }
    }
}

#[allow(dead_code)]
// Gradient function based on CSES counting bits problem
fn counting_bits_helper(n : u64) -> u64 {
    let mut ans : u64 = 0;
    for i in 0..((n as f64).log2() as i32) {
        let div : u64 = counting_bits_modpow(2, i as u64);
        let mut cnt : u64 = (n+1)/div;
        if (cnt % 2 == 0) {ans += cnt*div/2;}
        else {
            ans += (n+1)%div;
            cnt -= 1;
            ans += cnt*div/2;
        }
    }
    return ans;
}

#[allow(dead_code)]
// Binary exponentiation function for counting bits
fn counting_bits_modpow(x : u64, n : u64) -> u64 {
    if (n == 0) {return 1;}
    let mut u : u64 = counting_bits_modpow(x, n/2);
    u = (u * u);
    if (n % 2 == 1) {u = (u * x);}
    return u;
}

#[allow(dead_code)]
pub fn create_cses_images() {
    // Variable declarations
    let size : u32 = 750;

    // Attempting to create image directory
    fs::create_dir_all("image/cses").expect("Unable to create image/cses directory");

    // Number spiral image
    let mut curr1 : image::Image = image::Image::new_dimension(size, size);
    number_spiral(&mut curr1, 0.2 as f32, 0.2 as f32);
    file::create_file("image/cses/corridor.ppm", curr1);

    // Number grid image
    let mut curr2 : image::Image = image::Image::new_dimension(size, size);
    number_grid(&mut curr2, 1.0 as f32, 0.5 as f32);
    file::create_file("image/cses/checkerboard.ppm", curr2);

    // Counting bits image
    let mut curr3 : image::Image = image::Image::new_dimension(size, size);
    counting_bits(&mut curr3, 0.6 as f32, 0.6 as f32);
    file::create_file("image/cses/chains.ppm", curr3);
}
