// Imports
use std::fmt;

/// Pixel struct
#[derive(Clone, Copy)]
pub struct Pixel(u8, u8, u8);

// Implementing constructors
impl Pixel {
    /// New pixel with defualt values
    pub const fn new() -> Pixel {
        Pixel(0, 0, 0)
    }

    /// New pixel with values
    pub const fn new_value(r: u8, g: u8, b: u8) -> Pixel {
        Pixel(r, g, b)
    }
}

// Implementing mutators
impl Pixel {
    /// Updating all pixel values
    pub fn update(&mut self, r: u8, g: u8, b: u8) {
        self.0 = r;
        self.1 = g;
        self.2 = b;
    }

    /// Updating red value
    pub fn change_red(&mut self, val: u8) {
        self.0 = val;
    }

    /// Updating green value
    pub fn change_green(&mut self, val: u8) {
        self.1 = val;
    }

    /// Updating blue value
    pub fn change_blue(&mut self, val: u8) {
        self.2 = val;
    }
}

// Implementing accessors
impl Pixel {
    /// Getting red value
    pub fn get_red(&self) -> u8 {
        self.0
    }

    /// Getting green value
    pub fn get_green(&self) -> u8 {
        self.1
    }

    /// Getting blue value
    pub fn get_blue(&self) -> u8 {
        self.2
    }
}

// Implementing formatting for the struct
impl fmt::Display for Pixel {
    /// Function for formatted printing
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}  ", self.0, self.1, self.2)
    }
}

// Implementing comparison for the struct
impl PartialEq<Pixel> for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        (self.0 == other.0) && (self.1 == other.1) && (self.2 == other.2)
    }
}
