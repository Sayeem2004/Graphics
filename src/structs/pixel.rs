// Imports
use std::fmt;

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
