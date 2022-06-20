// Imports
use crate::template::prev::ver08::format::pixel::Pixel;
use std::fmt;

/// Image struct
pub struct Image {
    image_type: String,
    width: i32,
    height: i32,
    max_color: u8,
    pixels: Vec<Vec<Pixel>>,
    buff: Vec<Vec<f32>>,
}

// Implementing constructors
impl Image {
    /// New image with default values
    pub fn new() -> Image {
        Image {
            image_type: String::from("P3"),
            width: 500,
            height: 500,
            max_color: 255,
            pixels: vec![vec![Pixel::new(); 500]; 500],
            buff: vec![vec![f32::NEG_INFINITY; 500]; 500],
        }
    }

    /// New dimensional image
    pub fn new_dimension(width: i32, height: i32) -> Image {
        if (width < 0 || height < 0) {
            eprintln!("Image dimensions are out of range, using default image");
            return Image::new();
        }
        Image {
            image_type: String::from("P3"),
            width,
            height,
            max_color: 255,
            pixels: vec![vec![Pixel::new(); width as usize]; height as usize],
            buff: vec![vec![f32::NEG_INFINITY; width as usize]; height as usize],
        }
    }
}

// Implementing mutators
impl Image {
    /// Updating a certain pixel with another pixel
    pub fn update_pixel_xy(&mut self, x: i32, y: i32, z: f32, pix: Pixel) {
        // Within size range
        if (y >= 0 && y < self.height && x >= 0 && x < self.width) {
            // Within color range
            if (pix.get_red() <= self.max_color
                && pix.get_green() <= self.max_color
                && pix.get_blue() <= self.max_color)
            {
                // Updating pixel
                if (self.buff[((self.height) - 1 - y) as usize][x as usize] < z) {
                    self.pixels[((self.height) - 1 - y) as usize][x as usize].update(
                        pix.get_red(),
                        pix.get_green(),
                        pix.get_blue(),
                    );
                    self.buff[((self.height) - 1 - y) as usize][x as usize] = z;
                }
            }
        }
    }
}

// Implementing accessors
impl Image {
    /// Getting height
    pub fn get_height(&mut self) -> i32 {
        self.height
    }

    /// Getting width
    pub fn get_width(&mut self) -> i32 {
        self.width
    }
}

// Implementing formatting for the struct
impl fmt::Display for Image {
    /// Function for formatted printing
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Writing file information
        write!(
            f,
            "{}\n{} {}\n{}\n",
            self.image_type, self.width, self.height, self.max_color
        )?;

        // Iterating through pixels
        for row in self.pixels.iter() {
            for val in row.iter() {
                // Printing pixel
                write!(f, "{}", val)?;
            }
            // Printing new line
            writeln!(f)?;
        }

        // Printing final new line
        return writeln!(f);
    }
}

// Some basic utility functions for images
impl Image {
    /// Function to fill the entire image with a certain color
    pub fn fill(&mut self, pix: Pixel) {
        // Iterating through pixels
        for i in 0..self.height {
            for q in 0..self.width {
                // Updating pixel
                self.pixels[i as usize][q as usize] = pix;
            }
        }
    }

    /// Function that implements a gradient on an image given a gradient function
    pub fn gradient<F: Fn(i32, i32) -> Pixel>(&mut self, grad: F, z: f32) {
        // Iterating through pixels
        for y in 0..self.height {
            for x in 0..self.width {
                // Updating pixel
                self.update_pixel_xy(x, y, z, grad(x, y));
            }
        }
    }

    /// Function that resets the zbuffer
    pub fn reset_buff(&mut self) {
        // Iterating through image
        for i in 0..self.height {
            for q in 0..self.width {
                // Resetting zbuffer
                self.buff[i as usize][q as usize] = f32::NEG_INFINITY;
            }
        }
    }
}
