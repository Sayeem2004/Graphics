// Imports
use crate::prev::ver06::format::{constant, file, image::Image, pixel::Pixel};
use rand::Rng;
use std::{cmp::Ordering, fs};

/// Converts an image into the number spiral gradient
fn number_spiral(img: &mut Image, scale1: f32, scale2: f32) {
    // Error checking
    if (scale1 < 0.0 || scale2 < 0.0) {
        eprintln!("Scale factors for number spiral can not be negative, no changes made");
        return;
    }

    // Looping through pixels
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            // Initializing rgb colors
            let r: u8 = 0;
            let g: u8 =
                (number_spiral_helper(((i as f32) * scale1) as i32, ((q as f32) * scale1) as i32)
                    % 256) as u8;
            let b: u8 =
                (number_spiral_helper(((q as f32) * scale2) as i32, ((i as f32) * scale2) as i32)
                    % 256) as u8;

            // Updating pixel
            img.update_pixel_rc(i, q, Pixel::new_value(r, g, b));
        }
    }
}

/// Gradient function based on CSES number spiral problem
fn number_spiral_helper(row: i32, col: i32) -> i32 {
    // Error checking
    if (row < 0 || col < 0) {
        eprintln!("Indices for number spiral helper can not be negative, returning default value");
        return 0;
    }

    // Variable declarations
    let num: i32;

    // Casework
    match row.cmp(&col) {
        Ordering::Greater => {
            // Row major case
            if (row % 2 == 0) {
                num = row * row - col + 1;
            } else {
                num = (row - 1) * (row - 1) + col;
            }
        }
        Ordering::Less => {
            // Column major case
            if (col % 2 == 1) {
                num = col * col - row + 1;
            } else {
                num = (col - 1) * (col - 1) + row;
            }
        }
        Ordering::Equal => {
            // Neither case
            if (row % 2 == 0) {
                num = row * row - col + 1;
            } else {
                num = col * col - row + 1;
            }
        }
    }

    // Ending function
    num
}

/// Converts an image into the number grid gradient
fn number_grid(img: &mut Image, scale1: f32, scale2: f32) {
    // Error checking
    if (scale1 < 0.0 || scale2 < 0.0) {
        eprintln!("Scale factors for number grid can not be negative, no changes made");
        return;
    }

    // Looping through pixels
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            // Initializing rgb colors
            let r: u8 = 0;
            let g: u8 =
                (number_grid_helper(((i as f32) * scale1) as i32, ((q as f32) * scale1) as i32)
                    % 256) as u8;
            let b: u8 =
                (number_grid_helper(((q as f32) * scale2) as i32, ((i as f32) * scale2) as i32)
                    % 256) as u8;

            // Updating pixel
            img.update_pixel_rc(i, q, Pixel::new_value(r, g, b));
        }
    }
}

/// Gradient function based on CSES number grid problem
pub fn number_grid_helper(row: i32, col: i32) -> i32 {
    // Error checking
    if (row < 0 || col < 0) {
        eprintln!("Indices for number grid helper can not be negative, returning default value");
        return 0;
    }

    // Bitwise stuff
    ((row - 1) ^ (col - 1)).max(0)
}

/// Converts an image into a counting bits gradient
fn counting_bits(img: &mut Image, scale1: f32, scale2: f32) {
    // Error checking
    if (scale1 < 0.0 || scale2 < 0.0) {
        eprintln!("Scale factors for counting bits can not be negative, no changes made");
        return;
    }

    // Looping through pixels
    for i in 0..img.get_height() {
        for q in 0..img.get_width() {
            // Initializing rgb colors
            let r: u8 = 0;
            let g: u8 = (counting_bits_helper((((i ^ q) as f32) * scale1) as i64) % 256) as u8;
            let b: u8 = (counting_bits_helper((((i ^ q) as f32) * scale2) as i64) % 256) as u8;

            // Updating pixel
            img.update_pixel_rc(i, q, Pixel::new_value(r, g, b));
        }
    }
}

/// Gradient function based on CSES counting bits problem
fn counting_bits_helper(n: i64) -> i64 {
    // Error checking
    if (n < 0) {
        eprintln!("Index for counting bits helper can not be negative, returning default value");
        return 0;
    }

    // Variable declarations
    let mut ans: i64 = 0;

    // Looping through bits
    for i in 0..((n as f32).log2() as i32) {
        // Variable declarations
        let div: i64 = counting_bits_modpow(2, i as i64);
        let mut cnt: i64 = (n + 1) / div;

        // Casework for number of bits
        if (cnt % 2 == 0) {
            ans += cnt * div / 2;
        } else {
            ans += (n + 1) % div;
            cnt -= 1;
            ans += cnt * div / 2;
        }
    }

    // Ending function
    ans
}

/// Binary exponentiation function for counting bits
fn counting_bits_modpow(x: i64, n: i64) -> i64 {
    // Error checking
    if (x < 0 || n < 0) {
        eprintln!("Inputs for counting bits modpow can not be negative, returning default value");
        return 1;
    }

    // Base case
    if (n == 0) {
        return 1;
    }

    // Variable declaration
    let mut u: i64 = counting_bits_modpow(x, n / 2);
    u = (u * u);

    // Odd case
    if (n % 2 == 1) {
        u *= x;
    }

    // Ending function
    u
}

/// Function that creates the barnsley fern on an image
fn barnsley(img: &mut Image, pix: Pixel, itr: u32, scale: f32) {
    // Error checking
    if (scale < 0.0) {
        eprintln!("Scale factor for barnsley can not be negative, no changes made");
        return;
    }

    // Setting up random generator and array
    let mut rng = rand::thread_rng();
    let mut xval: Vec<f32> = vec![0.0; itr as usize];
    let mut yval: Vec<f32> = vec![0.0; itr as usize];

    // Looping through iterations
    for i in 1..itr {
        // Generating random number
        let prob: i32 = rng.gen_range(1..101);

        // Casework
        if (prob == 1) {
            // The stem
            xval[i as usize] = 0.0;
            yval[i as usize] = (0.16 * (yval[(i - 1) as usize]))
        } else if (prob <= 86) {
            // The small leaflet
            xval[i as usize] =
                ((0.85 * (xval[(i - 1) as usize])) + (0.04 * (yval[(i - 1) as usize])));
            yval[i as usize] = ((-0.04 * (xval[(i - 1) as usize]))
                + (0.85 * (yval[(i - 1) as usize]))
                + (img.get_width() as f32) / scale);
        } else if (prob <= 93) {
            // The large left leaflet
            xval[i as usize] =
                ((0.2 * (xval[(i - 1) as usize])) - (0.26 * (yval[(i - 1) as usize])));
            yval[i as usize] = ((0.23 * (xval[(i - 1) as usize]))
                + (0.22 * (yval[(i - 1) as usize]))
                + (img.get_width() as f32) / scale);
        } else {
            // The large right leaflet
            xval[i as usize] =
                ((-0.15 * (xval[(i - 1) as usize])) + (0.28 * (yval[(i - 1) as usize])));
            yval[i as usize] = ((0.26 * (xval[(i - 1) as usize]))
                + (0.24 * (yval[(i - 1) as usize]))
                + (img.get_width() as f32) / (scale * 4.0));
        }
    }

    // Iterating through coordinates
    for i in 0..itr {
        // Plotting points
        let width: i32 = img.get_width();
        img.update_pixel_xy(
            (xval[i as usize] + (width as f32 / (scale / 3.3))) as i32,
            yval[i as usize] as i32,
            pix,
        )
    }
}

/// Function that runs all the above pattern functions, 0 to open images, 1 to not open images
pub fn create_work00_images(mode: i32) {
    // Variable declarations
    let size: i32 = 750;

    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Number spiral image
    let mut curr1: Image = Image::new_dimension(size, size);
    number_spiral(&mut curr1, 0.2_f32, 0.2_f32);
    file::create_ppm_ascii("image/ppm/w00_corridor.ppm", &curr1, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w00_corridor.ppm");
    }

    // Number grid image
    let mut curr2: Image = Image::new_dimension(size, size);
    number_grid(&mut curr2, 1.0_f32, 0.5_f32);
    file::create_ppm_ascii("image/ppm/w00_checkerboard.ppm", &curr2, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w00_checkerboard.ppm");
    }

    // Counting bits image
    let mut curr3: Image = Image::new_dimension(size, size);
    counting_bits(&mut curr3, 0.6_f32, 0.6_f32);
    file::create_ppm_ascii("image/ppm/w00_chains.ppm", &curr3, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w00_chains.ppm");
    }

    // Barnsley fern drawing
    let mut curr4: Image = Image::new_dimension(size, size);
    barnsley(&mut curr4, constant::WHITE_PIXEL, 200000, 7.0);
    file::create_ppm_ascii("image/ppm/w00_barnsley.ppm", &curr4, 0);
    if (mode == 0) {
        file::open_image("image/ppm/w00_barnsley.ppm");
    }
}
