// Imports
use std::fmt;

/// Pixel struct
#[derive(Clone, Copy, Debug)]
pub struct Pixel(pub u8, pub u8, pub u8);

// Implementing constructors
impl Pixel {
    /// New pixel with defualt values
    pub const fn new() -> Pixel {
        Pixel(0, 0, 0)
    }

    /// New pixel based on scale
    pub fn new_scale(scale: f32) -> Pixel {
        Pixel(
            (255_f32 * scale) as u8,
            (255_f32 * scale) as u8,
            (255_f32 * scale) as u8,
        )
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
    // Function for equivalence
    fn eq(&self, other: &Pixel) -> bool {
        (self.0 == other.0) && (self.1 == other.1) && (self.2 == other.2)
    }
}
