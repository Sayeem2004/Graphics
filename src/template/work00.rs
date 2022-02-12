// Imports
use crate::format::{image::Image, file};
use crate::format::{pixel::Pixel, constant};
use rand::Rng;
use std::fs;

// Converts an image into the number spiral gradient
pub fn number_spiral(img : &mut Image, scale1: f32, scale2: f32) {
    // Looping through pixels
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            // Initializing rgb colors
            let r : u32 = 0;
            let g : u32 = number_spiral_helper(((i as f32) * scale1) as u32, ((q as f32) * scale1) as u32) % 256;
            let b : u32 = number_spiral_helper(((q as f32) * scale2) as u32, ((i as f32) * scale2) as u32) % 256;

            // Updating pixel
            img.update_pixel_rc(i, q, r, g, b);
        }
    }
}

// Gradient function based on CSES number spiral problem
fn number_spiral_helper(row : u32, col : u32) -> u32 {
    // Variable declarations
    let num : u32;

    // Casework
    if (row > col) {
        // Row major case
        if (row % 2 == 0) {num = row * row - col + 1;}
        else {num = (row - 1) * (row - 1) + col;}
    } else if (col > row) {
        // Column major case
        if (col % 2 == 1) {num = col * col - row + 1;}
        else {num = (col - 1) * (col - 1) + row;}
    } else {
        // Neither case
        if (row % 2 == 0) {num = row * row - col + 1;}
        else {num = col * col - row + 1;}
    }

    // Ending function
    return num;
}

// Converts an image into the number grid gradient
pub fn number_grid(img : &mut Image, scale1 : f32, scale2 : f32) {
    // Looping through pixels
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            // Initializing rgb colors
            let r : u32 = 0;
            let g : u32 = number_grid_helper(((i as f32) * scale1) as u32, ((q as f32) * scale1) as u32) % 256;
            let b : u32 = number_grid_helper(((q as f32) * scale2) as u32, ((i as f32) * scale2) as u32) % 256;

            // Updating pixel
            img.update_pixel_rc(i, q, r, g, b);
        }
    }
}

// Gradient function based on CSES number grid problem
fn number_grid_helper(row : u32, col : u32) -> u32 {
    // Bitwise stuff
    return (row-1)^(col-1);
}

// Converts an image into a counting bits gradient
pub fn counting_bits(img : &mut Image, scale1 : f32, scale2 : f32) {
    // Looping through pixels
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            // Initializing rgb colors
            let r : u32 = 0;
            let g : u32 = (counting_bits_helper((((i^q) as f32) * scale1) as u64) % 256) as u32;
            let b : u32 = (counting_bits_helper((((i^q) as f32) * scale2) as u64) % 256) as u32;

            // Updating pixel
            img.update_pixel_rc(i, q, r, g, b);
        }
    }
}

// Gradient function based on CSES counting bits problem
fn counting_bits_helper(n : u64) -> u64 {
    // Variable declarations
    let mut ans : u64 = 0;

    // Looping through bits
    for i in 0..((n as f64).log2() as i32) {
        // Variable declarations
        let div : u64 = counting_bits_modpow(2, i as u64);
        let mut cnt : u64 = (n+1)/div;

        // Casework for number of bits
        if (cnt % 2 == 0) {ans += cnt*div/2;}
        else {
            ans += (n+1)%div;
            cnt -= 1;
            ans += cnt*div/2;
        }
    }

    // Ending function
    return ans;
}

// Binary exponentiation function for counting bits
fn counting_bits_modpow(x : u64, n : u64) -> u64 {
    // Base case
    if (n == 0) {return 1;}

    // Variable declaration
    let mut u : u64 = counting_bits_modpow(x, n/2);
    u = (u * u);

    // Odd case
    if (n % 2 == 1) {u = (u * x);}

    // Ending function
    return u;
}

// Function that creates the barnsley fern on an image
pub fn barnsley(img : &mut Image, pix : Pixel, itr : u32, scale : f64) {
    // Setting up random generator and array
    let mut rng = rand::thread_rng();
    let mut xval : Vec<f64> = vec![0.0; itr as usize];
    let mut yval : Vec<f64> = vec![0.0; itr as usize];

    // Looping through iterations
    for i in 1..itr {
        // Generating random number
        let prob : i32 = rng.gen_range(1..101);

        // Casework
        if (prob == 1) {
            // The stem
            xval[i as usize] = 0.0;
            yval[i as usize] = (0.16 * (yval[(i-1) as usize]))
        } else if (prob <= 86) {
            // The small leaflet
            xval[i as usize] = ((0.85 * (xval[(i-1) as usize])) + (0.04 * (yval[(i-1) as usize])));
            yval[i as usize] = ((-0.04 * (xval[(i-1) as usize])) + (0.85 * (yval[(i-1) as usize])) + (img.get_width() as f64)/scale);
        } else if (prob <= 93) {
            // The large left leaflet
            xval[i as usize] = ((0.2 * (xval[(i-1) as usize])) - (0.26 * (yval[(i-1) as usize])));
            yval[i as usize] = ((0.23 * (xval[(i-1) as usize])) + (0.22 * (yval[(i-1) as usize])) + (img.get_width() as f64)/scale);
        } else {
            // The large right leaflet
            xval[i as usize] = ((-0.15 * (xval[(i-1) as usize])) + (0.28 * (yval[(i-1) as usize])));
            yval[i as usize] = ((0.26 * (xval[(i-1) as usize])) + (0.24 * (yval[(i-1) as usize])) + (img.get_width() as f64)/(scale*4.0));
        }
    }

    // Iterating through coordinates
    for i in 0..itr {
        // Plotting points
        let width : u32 = img.get_width();
        img.update_pixel_xy2((xval[i as usize] + ((width as f64/(scale/3.3)))) as u32, yval[i as usize] as u32, pix)
    }
}

// Function that runs all the above pattern functions
pub fn create_work00_images() {
    // Variable declarations
    let size : u32 = 750;

    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Number spiral image
    let mut curr1 : Image = Image::new_dimension(size, size);
    number_spiral(&mut curr1, 0.2 as f32, 0.2 as f32);
    file::create_ppm_ascii("image/ppm/w00_corridor.ppm", curr1);

    // Number grid image
    let mut curr2 : Image = Image::new_dimension(size, size);
    number_grid(&mut curr2, 1.0 as f32, 0.5 as f32);
    file::create_ppm_ascii("image/ppm/w00_checkerboard.ppm", curr2);

    // Counting bits image
    let mut curr3 : Image = Image::new_dimension(size, size);
    counting_bits(&mut curr3, 0.6 as f32, 0.6 as f32);
    file::create_ppm_ascii("image/ppm/w00_chains.ppm", curr3);

    // Barnsley fern drawing
    let mut curr4 : Image = Image::new_dimension(size, size);
    barnsley(&mut curr4, constant::WHITE_PIXEL, 200000, 7.0);
    file::create_ppm_ascii("image/ppm/w00_barnsley.ppm", curr4);
}
