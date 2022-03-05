// Imports
use crate::format::{parse, file};

pub fn create_work03_images(mode : i32) {
    // Creating test image
    parse::parse("src/data/w03/w03_script", 500);
    if (mode == 0) {file::open_image("image/usr/w03_pic.png");}
}
