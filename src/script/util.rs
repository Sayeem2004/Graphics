// Imports
use crate::compiler::{Operation, Symbol};
use crate::format::{constant, image::Image, util};
use crate::script::parse::ImageInfo;
use rand::{Rng, rngs::ThreadRng};
use std::{collections::HashMap, fs, process::Command};

/// Function that performs the 'basename' command
pub fn basename(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, info: &mut ImageInfo) {
    // Getting string argument
    let name: &String = (&op.args.as_ref().unwrap()[0]).as_string().unwrap();

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
    let mut rng: ThreadRng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..10000);
    let name: String = vec![
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
    let frames: i32 = *op.args.as_ref().unwrap()[0].as_float().unwrap() as i32;
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
    let mut rng: ThreadRng = rand::thread_rng();
    let num: i32 = rng.gen_range(0..10000);
    let name: String = vec![
        "temp/save".to_string(),
        format!("{:0>#4}", num),
        ".ppm".to_string(),
    ]
    .join("");
    util::create_ppm_ascii(&name, img, 1);

    // Getting image path
    let mut path: String = String::from("image/script/");
    let add: &String = op.args.as_ref().unwrap()[0].as_string().unwrap();
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

/// Function that performs the 'saveknobs' command
pub fn saveknobs(op: &Operation, symbols: &mut HashMap<String, Vec<Symbol>>) {
    // Variable declarations
    let name: String = (*op.knoblist0.as_ref().unwrap()).clone();
    let mut dict: Vec<Symbol> = vec![Symbol::String(String::from("knoblist"))];

    // Iterating through symbol table
    for (name, list) in symbols.iter() {
        // Checking to see if symbol is a knob and adding to it the stored dictionary
        if (list[0].as_string().unwrap().eq("knob")) {
            dict.push(Symbol::Dictionary((*name).clone(), *list[1].as_float().unwrap()));
        }
    }

    // Updating symbol table
    symbols.insert(name, dict);
}

/// Function that performs the 'set' command
pub fn set(op: &Operation, symbols: &mut HashMap<String, Vec<Symbol>>) {
    // Variable declarations
    let knob: String = (*op.knob.as_ref().unwrap()).clone();
    let val: f32 = *(&op.args.as_ref().unwrap()[0]).as_float().unwrap();

    // Error checking symbol table
    match symbols.get_mut(&knob) {
        None => {
            // Creating knob if symbol table value doesnt exist
            symbols.insert(
                knob,
                vec![Symbol::String(String::from("knob")), Symbol::Float(val)],
            );
        }
        Some(list) => {
            if (!list[0].as_string().unwrap().eq("knob")) {
                // If symbol table value exists but is not a knob
                eprintln!("Symbol value {} is not a knob, no changes made", knob);
            } else {
                // Updating symbol table if it exists and is a knob
                list[1] = Symbol::Float(val);
            }
        }
    }
}

/// Function that performs the 'setknobs' command
pub fn setknobs(op: &Operation, symbols: &mut HashMap<String, Vec<Symbol>>) {
    // Variable declarations
    let val: f32 = *(&op.args.as_ref().unwrap()[0]).as_float().unwrap();

    // Iterating through symbol table
    for (_, list) in symbols.iter_mut() {
        // Checking to see if symbol is not a knob
        if (!list[0].as_string().unwrap().eq("knob")) {
            continue;
        } else {
            list[1] = Symbol::Float(val);
        }
    }
}

/// Function that performs the 'screen' operation
pub fn screen(op: &Operation, info: &mut ImageInfo) {
    // Getting number arguments
    let width: i32 = *op.width.as_ref().unwrap() as i32;
    let height: i32 = *op.height.as_ref().unwrap() as i32;

    // Exiting function
    info.width = width;
    info.height = height;
}

/// Function that performs the 'tween' command
pub fn tween(op: &Operation, frames: &mut Vec<HashMap<String, f32>>, symbols: &HashMap<String, Vec<Symbol>>) {
    // Variable declarations
    let start: i32 = *(&op.args.as_ref().unwrap()[0]).as_float().unwrap() as i32;
    let end: i32 = *(&op.args.as_ref().unwrap()[1]).as_float().unwrap() as i32;
    let name1: String = (*op.knoblist0.as_ref().unwrap()).clone();
    let name2: String = (*op.knoblist1.as_ref().unwrap()).clone();

    // Error checking the knob lists
    let knoblist1: Vec<Symbol> = match symbols.get(&name1) {
        None => {
            eprintln!("Symbol {} is not found in the symbol table, using default value", name1);
            Vec::new()
        }
        Some(list) => {
            let typ: String = (*list[0].as_string().unwrap()).clone();
            if (!typ.eq("knoblist")) {
                eprintln!("Symbol {} is not a knoblist, using default value", name1);
                Vec::new()
            } else {
                (*list).clone()
            }
        }
    };
    let knoblist2: Vec<Symbol> = match symbols.get(&name2) {
        None => {
            eprintln!("Symbol {} is not found in the symbol table, using default value", name2);
            Vec::new()
        }
        Some(list) => {
            let typ: String = (*list[0].as_string().unwrap()).clone();
            if (!typ.eq("knoblist")) {
                eprintln!("Symbol {} is not a knoblist, using default value", name2);
                Vec::new()
            } else {
                (*list).clone()
            }
        }
    };

    // Iterating through the knoblists and compiling starting and ending values into a list
    let mut list: Vec<(String, f32, f32)> = Vec::new();
    for sym1 in knoblist1.iter().skip(1) {
        for sym2 in knoblist2.iter().skip(1) {
            // Checking if knobs are the same
            let knob1: String = (*sym1.as_dictionary().unwrap().0).clone();
            let knob2: String = (*sym2.as_dictionary().unwrap().0).clone();
            if (knob1.eq(&knob2)) {
                // Adding to list
                list.push((
                    knob1,
                    (*sym1.as_dictionary().unwrap().1),
                    (*sym2.as_dictionary().unwrap().1)
                ));
            }
        }
    }

    // Adding knob values from compiled list to frames list
    for val in list.iter() {
        for frame in 0..end-start+1 {
            let num: f32 = val.1 + (val.2 - val.1) * (frame as f32) / ((end - start) as f32);
            frames[frame as usize].insert(val.0.clone(), num);
        }
    }
}

/// Function that performs the 'vary' command
pub fn vary(op: &Operation, frames: &mut Vec<HashMap<String, f32>>) {
    // Variable declarations
    let knob: String = (*op.knob.as_ref().unwrap()).clone();
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
    let mut i: i32 = 0;
    let mx: i32 = end_frame - start_frame;
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
