// Imports
use std::convert::TryFrom;

// Function that safely converts u32 to i32
pub fn utoi(num : u32) -> i32 {
    match i32::try_from(num) {
        Ok(_) => {return i32::try_from(num).unwrap();},
        Err(x) => {println!("Conversion error from u32 to i32: {}", x); return 0;}
    };
}

// Function that safely converts i32 to u32
pub fn itou(num : i32) -> u32 {
    match u32::try_from(num) {
        Ok(_) => {return u32::try_from(num).unwrap();},
        Err(x) => {println!("Conversion error from i32 to u32: {}", x); return 0;}
    };
}
