// Imports
use std::fs::File;
use std::io::{BufWriter, Write};
mod structs; use structs::image;
mod patterns; use patterns::cses;

// Constants
const SIZE : u32 = 750;

// Main function
fn main() {
    // Making image struct
    let mut curr1 : image::Image = image::Image::new_dimension(SIZE, SIZE);
    cses::number_spiral(&mut curr1);

    // Making image file
    let file1 = File::create("image1.ppm").expect("Unable to create file");
    let mut writer1 = BufWriter::new(file1);
    writer1.write_all(format!("{}", curr1).as_bytes()).expect("Unable to write data");
    println!("Image file is named image1.ppm");

    // Making image struct
    let mut curr2 : image::Image = image::Image::new_dimension(SIZE, SIZE);
    cses::number_grid(&mut curr2);

    // Making image file
    let file2 = File::create("image2.ppm").expect("Unable to create file");
    let mut writer2 = BufWriter::new(file2);
    writer2.write_all(format!("{}", curr2).as_bytes()).expect("Unable to write data");
    println!("Image file is named image2.ppm");
}
