// Imports
use crate::template::prev::ver05::format::{constant, pixel::Pixel};
use std::{collections::VecDeque, fmt};

/// Image struct
pub struct Image {
    image_type: String,
    width: i32,
    height: i32,
    max_color: u8,
    pixels: Vec<Vec<Pixel>>,
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
        }
    }
}

// Implementing mutators
impl Image {
    /// Updating a certain pixel with another pixel
    pub fn update_pixel_rc(&mut self, row: i32, col: i32, pix: Pixel) {
        // Within size range
        if (row >= 0 && row < self.height && col >= 0 && col < self.width) {
            // Within color range
            if (pix.get_red() <= self.max_color
                && pix.get_green() <= self.max_color
                && pix.get_blue() <= self.max_color)
            {
                // Updating pixel
                self.pixels[row as usize][col as usize].update(
                    pix.get_red(),
                    pix.get_green(),
                    pix.get_blue(),
                );
            }
        }
    }

    /// Updating a certain pixel with another pixel
    pub fn update_pixel_xy(&mut self, x: i32, y: i32, pix: Pixel) {
        // Within size range
        if (y >= 0 && y < self.height && x >= 0 && x < self.width) {
            // Within color range
            if (pix.get_red() <= self.max_color
                && pix.get_green() <= self.max_color
                && pix.get_blue() <= self.max_color)
            {
                // Updating pixel
                self.pixels[((self.height) - 1 - y) as usize][x as usize].update(
                    pix.get_red(),
                    pix.get_green(),
                    pix.get_blue(),
                );
            }
        }
    }
}

// Implementing accessors
impl Image {
    /// Getting width
    pub fn get_width(&mut self) -> i32 {
        self.width
    }

    /// Getting height
    pub fn get_height(&mut self) -> i32 {
        self.height
    }

    /// Getting a certain pixel
    pub fn get_pixel_rc(&mut self, row: i32, col: i32) -> Pixel {
        // Within size range
        if (row >= 0 && row < self.height && col >= 0 && col < self.width) {
            // Getting pixel
            return self.pixels[row as usize][col as usize];
        }

        // Error message
        eprintln!("Unable to get pixel at row {} and col {}", row, col);

        // Ending function
        Pixel::new()
    }

    /// Getting a certain pixel
    pub fn get_pixel_xy(&mut self, x: i32, y: i32) -> Pixel {
        // Within size range
        if (y >= 0 && y < self.height && x >= 0 && x < self.width) {
            // Getting pixel
            return self.pixels[(self.height - y - 1) as usize][x as usize];
        }

        // Error message
        eprintln!("Unable to get pixel at x = {} and x = {}", x, y);

        // Ending function
        Pixel::new()
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

    /// Function to fill a certain section of the image with a certain color
    pub fn flood_xy(&mut self, x: i32, y: i32, bdr: Pixel, pix: Pixel) {
        // Within size range
        if (y >= 0 && y < self.height && x >= 0 && x < self.width) {
            // Making visited vector
            let mut vis = vec![vec![0; self.width as usize]; self.height as usize];

            // Creating queue and adding first element
            let mut queue = VecDeque::<(i32, i32)>::new();
            vis[y as usize][x as usize] = 1;
            queue.push_back((x, y));

            // Doing bfs
            while (!queue.is_empty()) {
                // Getting point
                let point: (i32, i32) = queue.pop_front().unwrap();
                self.update_pixel_xy(point.0, point.1, pix);

                for i in 0..4 {
                    // New points
                    let nx = (point.0) + constant::DX[i as usize];
                    let ny = (point.1) + constant::DY[i as usize];

                    // Within bounds
                    if (nx >= 0
                        && nx < self.width
                        && ny >= 0
                        && ny < self.height
                        && vis[ny as usize][nx as usize] == 0
                        && self.get_pixel_xy(nx, ny) != bdr
                        && self.get_pixel_xy(nx, ny) != pix)
                    {
                        // Adding to queue and visited
                        vis[ny as usize][nx as usize] = 1;
                        queue.push_back((nx, ny));
                    }
                }
            }

            return;
        }

        // Error message
        eprintln!("Unable to flood pixel at x = {} and y = {}", x, y);
    }

    /// Function to fill a certain section of the image with a certain color
    pub fn flood_rc(&mut self, row: i32, col: i32, bdr: Pixel, pix: Pixel) {
        // Within size range
        if (row >= 0 && row < self.height && col >= 0 && col < self.width) {
            // Making visited vector
            let mut vis = vec![vec![0; self.width as usize]; self.height as usize];

            // Creating queue and adding first element
            let mut queue = VecDeque::<(i32, i32)>::new();
            vis[row as usize][col as usize] = 1;
            queue.push_back((row, col));

            // Doing bfs
            while (!queue.is_empty()) {
                // Getting point
                let point: (i32, i32) = queue.pop_front().unwrap();
                self.update_pixel_rc(point.0, point.1, pix);

                for i in 0..4 {
                    // New points
                    let nrow = (point.0) + constant::DX[i as usize];
                    let ncol = (point.1) + constant::DY[i as usize];

                    // Within bounds
                    if (ncol >= 0
                        && ncol < self.width
                        && nrow >= 0
                        && nrow < self.height
                        && vis[nrow as usize][ncol as usize] == 0
                        && self.get_pixel_rc(nrow, ncol) != bdr
                        && self.get_pixel_rc(nrow, ncol) != pix)
                    {
                        // Adding to queue and visited
                        vis[nrow as usize][ncol as usize] = 1;
                        queue.push_back((nrow, ncol));
                    }
                }
            }

            return;
        }

        // Error message
        eprintln!("Unable to flood pixel at row = {} and col = {}", row, col);
    }

    /// Function that replaces all pixels of a certain type with another
    pub fn replace(&mut self, old: Pixel, new: Pixel) {
        // Iterating through image
        for i in 0..self.pixels.len() {
            for q in 0..self.pixels[0].len() {
                // Replacing pixels
                if (self.pixels[i][q] == old) {
                    self.pixels[i][q] = new;
                }
            }
        }
    }
}
