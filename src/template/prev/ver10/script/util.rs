// Imports
use crate::template::prev::ver10::format::{constant, image::Image, util};
use crate::template::prev::ver10::script::compile::Operation;
use rand::Rng;
use std::{fs, process::Command};

/// Function that returns the image to default black and clears the zbuffer
pub fn clear(img: &mut Image) {
    // Making the screen black
    img.fill(constant::BLACK_PIXEL);

    // Resetting zbuffer
    img.reset_buff();
}

/// Function that performs the 'display' operation
pub fn display(img: &Image, mode: i32) {
    // Error checking mode
    if (mode != 0) {
        return;
    }

    // Attempting to create image directory
    fs::create_dir_all("temp").expect("Unable to create temp directory");

    // Saving image
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..10000);
    let name = vec![
        "temp/display".to_string(),
        format!("{:0>#4}", num),
        ".ppm".to_string(),
    ]
    .join("");
    util::create_ppm_ascii(&name, img, 1);

    // Displaying image
    util::open_image(&name);

    // Removing temporary file
    fs::remove_file(&name).expect("Unable to delete temporary display file");
}

/// Function that performs the 'save' operation
pub fn save(op: &Operation, img: &Image) {
    // Attempting to create image directory
    fs::create_dir_all("temp").expect("Unable to create temp directory");
    fs::create_dir_all("image/script").expect("Unable to create image/script directory");

    // Saving image
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..10000);
    let name = vec![
        "temp/save".to_string(),
        format!("{:0>#4}", num),
        ".ppm".to_string(),
    ]
    .join("");
    util::create_ppm_ascii(&name, img, 1);

    // Getting image path
    let mut path = String::from("image/script/");
    let add = op.args.as_ref().unwrap()[0].as_string().unwrap();
    path.push_str(add);

    // Performing image magick convert command
    Command::new("convert")
        .arg(&name)
        .arg(&path)
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named {}", path);

    // Removing temporary file
    fs::remove_file(&name).expect("Unable to delete temporary save file");
}
