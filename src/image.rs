// Imports
use std::fmt;

#[allow(dead_code)]
// Image struct
pub struct Img {
    image_type : String,
    width : u32,
    height : u32,
    max_color : u32,
    pixels : Vec<Vec<Pixel>>
}

#[allow(dead_code)]
// Implementing constructors
impl Img {
    // New image with default values
    pub fn new() -> Img {
        return Img {image_type : String::from("P3"),
                    width : 500,
                    height : 500,
                    max_color : 255,
                    pixels : vec![vec![Pixel::new(); 500]; 500]
        };
    }

    // New dimensional image
    pub fn new_dimension(width : u32, height : u32) -> Img {
        return Img {image_type : String::from("P3"),
                    width : width,
                    height : height,
                    max_color : 255,
                    pixels : vec![vec![Pixel::new(); width as usize]; height as usize]
        };
    }

    // New formatted image
    pub fn new_format(image_type : &str, width : u32, height : u32, max_color : u32) -> Img {
        return Img {image_type : image_type.to_string(),
                    width : width,
                    height : height,
                    max_color : max_color,
                    pixels : vec![vec![Pixel::new(); width as usize]; height as usize]
        };
    }
}

#[allow(dead_code)]
// Implementing mutators
impl Img {
    // Changing image type
    pub fn update_image_type(&mut self, image_type : &str) {
        self.image_type = image_type.to_string();
    }

    // Changing width
    pub fn update_width(&mut self, width : u32) {
        self.width = width;
        for row in self.pixels.iter_mut() {
            row.resize_with(self.width as usize, || Pixel::new());
        }
    }

    // Changing height
    pub fn update_height(&mut self, height : u32) {
        self.height = height;
        self.pixels.resize_with(self.height as usize, || vec![Pixel::new(); self.width as usize]);
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
impl Img {
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
    pub fn get_pixel(&self, row : u32, col : u32) -> Pixel {
        if row < self.height && col < self.width {
            return self.pixels[row as usize][col as usize];
        }
        println!("Unable to get pixel at row {} and col {}", row, col);
        return Pixel::new();
    }
}

// Implementing formatting for the struct
impl fmt::Display for Img {
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

#[allow(dead_code)]
#[derive(Clone, Copy)]
// Pixel struct
pub struct Pixel(u32, u32, u32);

#[allow(dead_code)]
// Implementing constructors
impl Pixel {
    // New pixel with defualt values
    pub fn new() -> Pixel {
        return Pixel {0 : 0, 1 : 0, 2 : 0};
    }

    // New pixel with values
    pub fn new_value(r : u32, g : u32, b : u32) -> Pixel {
        return Pixel {0 : r, 1 : g, 2 : b};
    }
}

#[allow(dead_code)]
// Implementing mutators
impl Pixel {
    // Updating all pixel values
    pub fn update(&mut self, r : u32, g : u32, b : u32) {
        self.0 = r;
        self.1 = g;
        self.2 = b;
    }

    // Updating red value
    pub fn change_red(&mut self, val : u32) {
        self.0 = val;
    }

    // Updating green value
    pub fn change_green(&mut self, val : u32) {
        self.1 = val;
    }

    // Updating blue value
    pub fn change_blue(&mut self, val : u32) {
        self.2 = val;
    }
}

#[allow(dead_code)]
// Implementing accessors
impl Pixel {
    // Getting red value
    pub fn get_red(&self) -> u32 {
        return self.0;
    }

    // Getting green value
    pub fn get_green(&self) -> u32 {
        return self.1;
    }

    // Getting blue value
    pub fn get_blue(&self) -> u32 {
        return self.2;
    }
}

// Implementing formatting for the struct
impl fmt::Display for Pixel {
    // Function for formatted printing
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{} {} {}  ", self.0, self.1, self.2);
    }
}
