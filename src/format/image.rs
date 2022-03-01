// Imports
use crate::format::pixel::Pixel;
use std::fmt;

/// Image struct
pub struct Image {
    image_type : String,
    width : i32,
    height : i32,
    max_color : u8,
    pixels : Vec<Vec<Pixel>>
}

// Implementing constructors
impl Image {
    /// New image with default values
    pub fn new() -> Image {
        return Image {
            image_type : String::from("P3"),
            width : 500,
            height : 500,
            max_color : 255,
            pixels : vec![vec![Pixel::new(); 500]; 500]
        };
    }

    /// New dimensional image
    pub fn new_dimension(width : i32, height : i32) -> Image {
        if (width < 0 || height < 0) {
            println!("Image dimensions are out of range, using default image");
            return Image::new();
        }
        return Image {
            image_type : String::from("P3"),
            width : width,
            height : height,
            max_color : 255,
            pixels : vec![vec![Pixel::new(); width as usize]; height as usize]
        };
    }

    /// New formatted image
    pub fn new_format(image_type : &str, width : i32, height : i32, max_color : u8) -> Image {
        if (width < 0 || height < 0) {
            println!("Image dimensions are out of range, using default image");
            return Image::new();
        }
        return Image {
            image_type : image_type.to_string(),
            width : width,
            height : height,
            max_color : max_color,
            pixels : vec![vec![Pixel::new(); width as usize]; height as usize]
        };
    }
}

// Implementing mutators
impl Image {
    /// Changing image type
    pub fn update_image_type(&mut self, image_type : &str) {
        self.image_type = image_type.to_string();
    }

    /// Changing width
    pub fn update_width(&mut self, width : i32) {
        if (width < 0) {
            println!("Image width is out of range, no changes made");
            return;
        }
        self.width = width;
        for row in self.pixels.iter_mut() {
            row.resize_with(self.width as usize, || Pixel::new());
        }
    }

    /// Changing height
    pub fn update_height(&mut self, height : i32) {
        if (height < 0) {
            println!("Image height is out of range, no changes made");
            return;
        }
        self.height = height;
        self.pixels.resize_with(self.height as usize, || vec![Pixel::new(); self.width as usize]);
    }

    /// Changing max color
    pub fn update_max_color(&mut self, max_color : u8) {
        self.max_color = max_color;
    }

    /// Updating a certain pixel with another pixel
    pub fn update_pixel_rc(&mut self, row : i32, col : i32, pix : Pixel) {
        // Within size range
        if (row >= 0 && row < self.height && col >= 0 && col < self.width) {
            // Within color range
            if (pix.get_red() <= self.max_color && pix.get_green() <= self.max_color && pix.get_blue() <= self.max_color) {
                // Updating pixel
                self.pixels[row as usize][col as usize].update(pix.get_red(), pix.get_green(), pix.get_blue());
                return;
            }
        }

        // Error message
        println!("Unable to update pixel at row {} and col {}", row, col);
    }

    /// Updating a certain pixel with another pixel
    pub fn update_pixel_xy(&mut self, x : i32, y : i32, pix : Pixel) {
        // Within size range
        if (y >= 0 && y < self.height && x >= 0 && x < self.width) {
            // Within color range
            if (pix.get_red() <= self.max_color && pix.get_green() <= self.max_color && pix.get_blue() <= self.max_color) {
                // Updating pixel
                self.pixels[((self.height)-1-y) as usize][x as usize].update(pix.get_red(), pix.get_green(), pix.get_blue());
                return;
            }
        }

        // Error message
        println!("Unable to update pixel at x = {} and y = {}", x, y);
    }
}

// Implementing accessors
impl Image {
    /// Getting image type
    pub fn get_image_type(&mut self) -> String {
        return self.image_type.clone();
    }

    /// Getting width
    pub fn get_width(&mut self) -> i32 {
        return self.width;
    }

    /// Getting height
    pub fn get_height(&mut self) -> i32 {
        return self.height;
    }

    /// Getting max color
    pub fn get_max_color(&mut self) -> u8 {
        return self.max_color;
    }

    /// Getting a certain pixel
    pub fn get_pixel_rc(&mut self, row : i32, col : i32) -> Pixel {
        // Within size range
        if (row >= 0 && row < self.height && col >= 0 && col < self.width) {
            // Getting pixel
            return self.pixels[row as usize][col as usize];
        }

        // Error message
        println!("Unable to get pixel at row {} and col {}", row, col);

        // Ending function
        return Pixel::new();
    }

    /// Getting a certain pixel
    pub fn get_pixel_xy(&mut self, x : i32, y : i32) -> Pixel {
        // Within size range
        if (y >= 0 && y < self.height && x >= 0 && x < self.width) {
            // Getting pixel
            return self.pixels[y as usize][x as usize];
        }

        // Error message
        println!("Unable to get pixel at x = {} and x = {}", x, y);

        // Ending function
        return Pixel::new();
    }
}

// Implementing formatting for the struct
impl fmt::Display for Image {
    /// Function for formatted printing
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        // Writing file information
        write!(f, "{}\n{} {}\n{}\n", self.image_type, self.width, self.height, self.max_color)?;

        // Iterating through pixels
        for row in self.pixels.iter() {
            for val in row.iter() {
                // Printing pixel
                write!(f, "{}", val)?;
            }
            // Printing new line
            write!(f, "\n")?;
        }

        // Printing final new line
        return write!(f, "\n");
    }
}

// Some basic utility functions for images
impl Image {
    /// Function to fill the entire image with a certain color
    pub fn fill(&mut self, pix : Pixel) {
        // Iterating through pixels
        for i in 0..self.height {
            for q in 0..self.width {
                // Updating pixel
                self.pixels[i as usize][q as usize] = pix;
            }
        }
    }
}
