// Imports
use std::fs::File;
use std::io::{BufWriter, Write};
mod image; mod pattern;

// Constants
const SIZE : u32 = 750;

// Main function
fn main() {
    // Making image struct
    let mut curr1 : image::Img = image::Img::new_dimension(SIZE, SIZE);

    // Making color gradient / style
    for i in 0..SIZE {
        for q in 0..SIZE {
            let r : u32 = 0;
            let g : u32 = pattern::number_spiral(i/5, q/5) % 256;
            let b : u32 = pattern::number_spiral(q/5, i/5) % 256;
            curr1.update_pixel(i, q, r, g, b);
        }
    }

    // Making image file
    let file1 = File::create("image1.ppm").expect("Unable to create file");
    let mut writer1 = BufWriter::new(file1);
    writer1.write_all(format!("{}", curr1).as_bytes()).expect("Unable to write data");
    println!("Image file is named image1.ppm");

    // Making image struct
    let mut curr2 : image::Img = image::Img::new_dimension(SIZE, SIZE);

    // Making color gradient / style
    for i in 0..SIZE {
        for q in 0..SIZE {
            let r : u32 = 0;
            let g : u32 = pattern::number_grid(i, q) % 256;
            let b : u32 = pattern::number_grid(q/2, i/2) % 256;
            curr2.update_pixel(i, q, r, g, b);
        }
    }

    // Making image file
    let file2 = File::create("image2.ppm").expect("Unable to create file");
    let mut writer2 = BufWriter::new(file2);
    writer2.write_all(format!("{}", curr2).as_bytes()).expect("Unable to write data");
    println!("Image file is named image2.ppm");
}
