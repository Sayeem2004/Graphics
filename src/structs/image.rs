// Imports
use std::fmt;
use crate::structs::pixel;

#[allow(dead_code)]
// Image struct
pub struct Image {
    image_type : String,
    width : u32,
    height : u32,
    max_color : u32,
    pixels : Vec<Vec<pixel::Pixel>>
}

#[allow(dead_code)]
// Implementing constructors
impl Image {
    // New image with default values
    pub fn new() -> Image {
        return Image {image_type : String::from("P3"),
                    width : 500,
                    height : 500,
                    max_color : 255,
                    pixels : vec![vec![pixel::Pixel::new(); 500]; 500]
        };
    }

    // New dimensional image
    pub fn new_dimension(width : u32, height : u32) -> Image {
        return Image {image_type : String::from("P3"),
                    width : width,
                    height : height,
                    max_color : 255,
                    pixels : vec![vec![pixel::Pixel::new(); width as usize]; height as usize]
        };
    }

    // New formatted image
    pub fn new_format(image_type : &str, width : u32, height : u32, max_color : u32) -> Image {
        return Image {image_type : image_type.to_string(),
                    width : width,
                    height : height,
                    max_color : max_color,
                    pixels : vec![vec![pixel::Pixel::new(); width as usize]; height as usize]
        };
    }
}

#[allow(dead_code)]
// Implementing mutators
impl Image {
    // Changing image type
    pub fn update_image_type(&mut self, image_type : &str) {
        self.image_type = image_type.to_string();
    }

    // Changing width
    pub fn update_width(&mut self, width : u32) {
        self.width = width;
        for row in self.pixels.iter_mut() {
            row.resize_with(self.width as usize, || pixel::Pixel::new());
        }
    }

    // Changing height
    pub fn update_height(&mut self, height : u32) {
        self.height = height;
        self.pixels.resize_with(self.height as usize, || vec![pixel::Pixel::new(); self.width as usize]);
    }

    // Changing max color
    pub fn update_max_color(&mut self, max_color : u32) {
        self.max_color = max_color;
    }

    // Updating a certain pixel
    pub fn update_pixel(&mut self, row : u32, col : u32, r : u32, g : u32, b : u32) {
        if row < self.height && col < self.width {
            if r <= self.max_color && g <= self.max_color && b <= self.max_color {
                self.pixels[row as usize][col as usize].update(r, g, b);
                return;
            }
        }
        println!("Unable to update pixel at row {} and col {}", row, col);
    }
}


#[allow(dead_code)]
// Implementing accessors
impl Image {
    // Getting image type
    pub fn get_image_type(&self) -> String {
        return self.image_type.clone();
    }

    // Getting width
    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    // Getting height
    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    // Getting max color
    pub fn get_max_color(&self) -> u32 {
        return self.max_color;
    }

    // Getting a certain pixel
    pub fn get_pixel(&self, row : u32, col : u32) -> pixel::Pixel {
        if row < self.height && col < self.width {
            return self.pixels[row as usize][col as usize];
        }
        println!("Unable to get pixel at row {} and col {}", row, col);
        return pixel::Pixel::new();
    }
}

// Implementing formatting for the struct
impl fmt::Display for Image {
    // Function for formatted printing
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{} {}\n{}\n", self.image_type, self.width, self.height, self.max_color)?;
        for row in self.pixels.iter() {
            for val in row.iter() {
                write!(f, "{}", val)?;
            }
            write!(f, "\n")?;
        }
        return write!(f, "\n");
    }
}
