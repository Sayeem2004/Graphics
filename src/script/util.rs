// Imports
use crate::compiler::{Operation, Symbol};
use crate::format::{constant, image::Image, util};
use crate::script::parse::ImageInfo;
use rand::Rng;
use std::{collections::HashMap, fs, process::Command};

/// Function that performs the 'basename' command
pub fn basename(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, info: &mut ImageInfo) {
    // Getting string argument
    let name = (&op.args.as_ref().unwrap()[0]).as_string().unwrap();

    // Checking if it is in symbol table
    match symbols.get(name) {
        // If name is not in symbol table just use the string input as basename
        None => {
            info.basename = Some((*name).clone());
        }

        // if name is in symbol table try to use the symbol value as the basename
        Some(list) => {
            if (list.len() <= 1) {
                eprintln!(
                    "Symbol {} has a descriptor but no value, using default value",
                    *name
                );
                info.basename = Some((*name).clone());
            } else {
                info.basename = Some((*list[1].as_string().unwrap()).clone());
            }
        }
    }
}

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

/// Function that performs the 'frames' operation
pub fn frames(op: &Operation, info: &mut ImageInfo) {
    // Error checking
    let frames = *op.args.as_ref().unwrap()[0].as_float().unwrap() as i32;
    if (frames < 1) {
        eprintln!("Number of frames specified is less than 1, using default value");
        info.num_frames = info.num_frames.max(frames);
        return;
    }

    // Exiting function
    info.num_frames = frames;
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

/// Function that performs the 'screen' operation
pub fn screen(op: &Operation, info: &mut ImageInfo) {
    // // Getting number arguments
    let width: i32 = *op.width.as_ref().unwrap() as i32;
    let height: i32 = *op.height.as_ref().unwrap() as i32;

    // Exiting function
    info.width = width;
    info.height = height;
}

/// Function that performs the 'vary' command
pub fn vary(op: &Operation, frames: &mut Vec<HashMap<String, f32>>) {
    // Error checking knob
    if (op.knob == None) {
        eprintln!("Knob name is empty, no changes made");
        return;
    }
    let knob = (*op.knob.as_ref().unwrap()).clone();

    // Getting number arguments
    let mut start_frame: i32 = *(&op.args.as_ref().unwrap()[0]).as_float().unwrap() as i32;
    let mut end_frame: i32 = *(&op.args.as_ref().unwrap()[1]).as_float().unwrap() as i32;
    let start_val: f32 = *(&op.args.as_ref().unwrap()[2]).as_float().unwrap();
    let end_val: f32 = *(&op.args.as_ref().unwrap()[3]).as_float().unwrap();

    // Error checking frame values
    if (start_frame < 0) {
        eprintln!("Start frame is negative, using default value");
        start_frame = 0;
    }
    if (end_frame >= (frames.len()) as i32) {
        eprintln!("End frame is greater than number of frames, using default value");
        end_frame = (frames.len() - 1) as i32;
    }

    // Adding knob values to vector
    let mut i = 0;
    let mx = end_frame - start_frame;
    while (i <= mx) {
        // Adding value to vector
        frames[(start_frame + i) as usize].insert(
            knob.clone(),
            start_val + (end_val - start_val) * (i as f32 / mx as f32),
        );

        // Iterating once
        i += 1;
    }
}
