use crate::compiler::{Operation, Symbol};
use crate::format::{constant, image::Image, matrix::Matrix, util::create_ppm_ascii};
use crate::script::{curve, light, shape, transform, util};
use std::{collections::HashMap, fs, process::Command};

pub struct ImageInfo {
    pub width: i32,
    pub height: i32,
    pub num_frames: i32,
    pub basename: Option<String>,
    pub curr_frame: i32,
    pub animate: bool,
    pub points: Vec<String>,
}

/// Function that performs and initial parse and obtains information about script
pub fn initial_parse(
    operations: &Vec<Operation>,
    symbols: &mut HashMap<String, Vec<Symbol>>,
) -> ImageInfo {
    // Indexing variables
    let mx: usize = operations.len();
    let mut curr: usize = 0;

    // Initializing image/animate info struct
    let mut info: ImageInfo = ImageInfo {
        width: 750,
        height: 750,
        num_frames: 1,
        basename: None,
        curr_frame: 0,
        animate: false,
        points: Vec::new(),
    };

    // Creating coordinate stack and list of point sources
    let mut stack: Vec<Matrix> = vec![Matrix::new_transformation()];
    let mut sz: usize = stack.len();

    // Looping through all operations
    while (curr < mx) {
        match operations[curr].op.as_ref().unwrap().as_str() {
            // Ambient command
            "ambient" => {
                light::ambient(symbols);
                curr += 1;
            }

            // Basename command
            "basename" => {
                util::basename(&operations[curr], symbols, &mut info);
                curr += 1;
            }

            // Constants command
            "constants" => {
                light::constants(&operations[curr], symbols);
                curr += 1;
            }

            // Frames command
            "frames" => {
                util::frames(&operations[curr], &mut info);
                curr += 1;
            }

            // Light command
            "light" => {
                light::light(&operations[curr], symbols, &mut info);
                curr += 1;
            }

            // Move command
            "move" => {
                transform::_move(&operations[curr], symbols, &mut stack[sz - 1]);
                curr += 1;
            }

            // Pop command
            "pop" => {
                transform::pop(&mut stack, &mut sz);
                curr += 1;
            }

            // Push command
            "push" => {
                transform::push(&mut stack, &mut sz);
                curr += 1;
            }

            // Rotate command
            "rotate" => {
                transform::rotate(&operations[curr], symbols, &mut stack[sz - 1]);
                curr += 1;
            }

            // Savecs command
            "savecs" => {
                transform::savecs(&operations[curr], symbols, &mut stack[sz - 1]);
                curr += 1;
            }

            // Scale command
            "scale" => {
                transform::scale(&operations[curr], symbols, &mut stack[sz - 1]);
                curr += 1;
            }

            // Screen command
            "screen" => {
                util::screen(&operations[curr], &mut info);
                curr += 1;
            }

            // Default case
            name => {
                if (!constant::CMDS.contains(&operations[curr].op.as_ref().unwrap().as_str())) {
                    eprintln!("The command \'{}\' is invalid", name);
                }
                curr += 1;
            }
        }
    }

    // Exiting function
    info
}

/// Function that sets up knob values for all frames
pub fn vary_parse(operations: &Vec<Operation>, info: &ImageInfo) -> Vec<HashMap<String, f32>> {
    // Indexing variables
    let mx: usize = operations.len();
    let mut curr: usize = 0;

    // Initializing frames vector
    let mut frames: Vec<HashMap<String, f32>> = vec![HashMap::new(); info.num_frames as usize];

    // Looping through all operations
    while (curr < mx) {
        match operations[curr].op.as_ref().unwrap().as_str() {
            // Vary command
            "vary" => {
                util::vary(&operations[curr], &mut frames);
                curr += 1;
            }

            // Default case
            _ => {
                curr += 1;
            }
        }
    }

    // Exiting function
    frames
}

/// Function that parses an operations list, symbol table, and frames list and creates a gif.
pub fn animate_parse(
    operations: &Vec<Operation>,
    symbols: &mut HashMap<String, Vec<Symbol>>,
    info: &mut ImageInfo,
    frames: Vec<HashMap<String, f32>>,
    mode: i32,
) {
    // Looping through frames
    for map in &frames {
        // Updating symbols table
        for (knob, val) in map {
            // Error checking symbol table
            match symbols.get_mut(knob) {
                None => {
                    eprintln!(
                        "Knob {} could not be found in symbol table, no changes made",
                        knob
                    );
                }
                Some(list) => {
                    if (!list[0].as_string().unwrap().eq("knob")) {
                        eprintln!("Symbol value {} is not a knob, no changes made", knob);
                    } else {
                        list[1] = Symbol::Float(*val);
                    }
                }
            }
        }

        // Sending update stuff to be drawn
        draw_parse(operations, symbols, info, mode);
        info.curr_frame += 1;
    }

    // Compiling into a gif using image magick
    let basename = (*info.basename.as_ref().unwrap()).clone();
    println!("Converting {0} images to {0}.gif...", basename);
    Command::new("convert")
        .arg("-delay")
        .arg("1.7")
        .arg("-loop")
        .arg("0")
        .arg(format!("temp/{}*.ppm", basename))
        .arg(format!("image/gif/{}.gif", basename))
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named image/gif/{}.gif", basename);

    // Removing temporary images
    for i in 0..frames.len() {
        let name = vec![
            "temp/".to_string(),
            basename.clone(),
            format!("{:0>#3}", i),
            ".ppm".to_string(),
        ]
        .join("");
        fs::remove_file(&name).expect("Unable to delete temporary gif file");
    }
}

/// Function that parses an operations list and a symbol table and draws an image.
pub fn draw_parse(
    operations: &Vec<Operation>,
    symbols: &mut HashMap<String, Vec<Symbol>>,
    info: &ImageInfo,
    mode: i32,
) {
    // Indexing variables
    let mx: usize = operations.len();
    let mut curr: usize = 0;

    // Creating coordinate stack, image struct, and source list
    let mut stack: Vec<Matrix> = vec![Matrix::new_transformation()];
    let mut sz: usize = stack.len();
    let mut img: Image = Image::new_dimension(info.width, info.height);

    // Looping through all operations
    while (curr < mx) {
        match operations[curr].op.as_ref().unwrap().as_str() {
            // Bezier command
            "bezier" => {
                curve::bezier(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Box command
            "box" => {
                shape::_box(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    info,
                    &mut img,
                );
                curr += 1;
            }

            // Circle command
            "circle" => {
                curve::circle(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Clear command
            "clear" => {
                util::clear(&mut img);
                curr += 1;
            }

            // Display command
            "display" => {
                util::display(&img, mode);
                curr += 1;
            }

            // Hermite command
            "hermite" => {
                curve::hermite(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Line command
            "line" => {
                curve::line(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Move command
            "move" => {
                transform::_move(&operations[curr], symbols, &mut stack[sz - 1]);
                curr += 1;
            }

            // Pop command
            "pop" => {
                transform::pop(&mut stack, &mut sz);
                curr += 1;
            }

            // Push command
            "push" => {
                transform::push(&mut stack, &mut sz);
                curr += 1;
            }

            // Rotate command
            "rotate" => {
                transform::rotate(&operations[curr], symbols, &mut stack[sz - 1]);
                curr += 1;
            }

            // Save command
            "save" => {
                util::save(&operations[curr], &img);
                curr += 1;
            }

            // Scale command
            "scale" => {
                transform::scale(&operations[curr], symbols, &mut stack[sz - 1]);
                curr += 1;
            }

            // Sphere command
            "sphere" => {
                shape::sphere(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    info,
                    &mut img,
                );
                curr += 1;
            }

            // Torus command
            "torus" => {
                shape::torus(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    info,
                    &mut img,
                );
                curr += 1;
            }

            // Default case
            _ => {
                curr += 1;
            }
        }
    }

    // Saving frame if animate is true
    if (info.animate) {
        let name = vec![
            "temp/".to_string(),
            (*info.basename.as_ref().unwrap()).clone(),
            format!("{:0>#3}", info.curr_frame),
            ".ppm".to_string(),
        ]
        .join("");
        create_ppm_ascii(&name, &img, 0);
    }
}
