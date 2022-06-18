// Imports
use crate::algorithm::extra;
use crate::compiler::{Argument, Operation, Symbol};
use crate::format::{image::Image, matrix::Matrix};
use std::collections::HashMap;

/// Function that performs the 'move' command
pub fn _move(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, cord: &mut Matrix) {
    // Getting number arguments
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut x: f32 = *args[0].as_float().unwrap();
    let mut y: f32 = *args[1].as_float().unwrap();
    let mut z: f32 = *args[2].as_float().unwrap();

    // Checking for knob
    let mut fnd: bool = true;
    let mut knob: String = String::new();
    match op.knob {
        None => {
            fnd = false;
        }
        _ => {
            knob = (*op.knob.as_ref().unwrap()).clone();
        }
    }

    // Updating values using knob
    (x, y, z) = if (fnd) {
        match symbols.get(&knob) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, no changes made",
                    knob
                );
                (x, y, z)
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("knob")) {
                    eprintln!("Symbol value {} is not a knob, no changes made", knob);
                    (x, y, z)
                } else {
                    let scale: f32 = *list[1].as_float().unwrap();
                    (x * scale, y * scale, z * scale)
                }
            }
        }
    } else {
        (x, y, z)
    };

    // Getting translation matrix and performing translation
    let trans: Matrix = Matrix::translate(x, y, z);
    cord.right_transform(&trans);
}

/// Function that performs the 'pop' operation
pub fn pop(stack: &mut Vec<Matrix>, sz: &mut usize) {
    // Removing top of stack
    stack.pop();

    // Updating stack size
    *sz -= 1;
}

/// Function that performs the 'push' operation
pub fn push(stack: &mut Vec<Matrix>, sz: &mut usize) {
    // Making copy of top
    let copy: Matrix = stack[*sz - 1].clone();

    // Adding copy to top of stack
    stack.push(copy);

    // Updating stack size
    *sz += 1;
}

/// Function that performs the 'rotate' command
pub fn rotate(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, cord: &mut Matrix) {
    // Getting number arguments
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut ang: f32 = *args[1].as_float().unwrap();

    // Checking for knob
    let mut fnd: bool = true;
    let mut knob: String = String::new();
    match op.knob {
        None => {
            fnd = false;
        }
        _ => {
            knob = (*op.knob.as_ref().unwrap()).clone();
        }
    }

    // Updating values using knob
    ang = if (fnd) {
        match symbols.get(&knob) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, no changes made",
                    knob
                );
                ang
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("knob")) {
                    eprintln!("Symbol value {} is not a knob, no changes made", knob);
                    ang
                } else {
                    let scale: f32 = *list[1].as_float().unwrap();
                    ang * scale
                }
            }
        }
    } else {
        ang
    };

    // Getting rotation matrix and performing rotation
    let trans: Matrix = Matrix::rotate_degree(ang, args[0].as_string().unwrap());
    cord.right_transform(&trans);
}

/// Function that performs the 'savecs' command
pub fn savecs(op: &Operation, symbols: &mut HashMap<String, Vec<Symbol>>, cord: &mut Matrix) {
    // Getting coordinate system name
    let name: String = (*op.cs0.as_ref().unwrap()).clone();

    // Updating symbol table entry
    match symbols.get(&name) {
        None => {
            eprintln!(
                "Symbol {} could not be found in symbol table, no changes made",
                name
            );
        }
        Some(list) => {
            if (!list[0].as_string().unwrap().eq("cs")) {
                eprintln!(
                    "Symbol value {} is not a coordinate system, no changes made",
                    name
                );
            } else {
                symbols.insert(
                    name,
                    vec![
                        Symbol::String(String::from("cs")),
                        Symbol::Matrix((*cord).clone()),
                    ],
                );
            }
        }
    }
}

/// Function that performs the 'scale' command
pub fn scale(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, cord: &mut Matrix) {
    // Getting number arguments
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut x: f32 = *args[0].as_float().unwrap();
    let mut y: f32 = *args[1].as_float().unwrap();
    let mut z: f32 = *args[2].as_float().unwrap();

    // Checking for knob
    let mut fnd: bool = true;
    let mut knob: String = String::new();
    match op.knob {
        None => {
            fnd = false;
        }
        _ => {
            knob = (*op.knob.as_ref().unwrap()).clone();
        }
    }

    // Updating values using knob
    (x, y, z) = if (fnd) {
        match symbols.get(&knob) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, no changes made",
                    knob
                );
                (x, y, z)
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("knob")) {
                    eprintln!("Symbol value {} is not a knob, no changes made", knob);
                    (x, y, z)
                } else {
                    let scale: f32 = *list[1].as_float().unwrap();
                    (x * scale, y * scale, z * scale)
                }
            }
        }
    } else {
        (x, y, z)
    };

    // Getting dilation matrix and performing dilation
    let trans: Matrix = Matrix::dilate(x, y, z);
    cord.right_transform(&trans);
}

/// Function that performs the terrain command
pub fn terrain(op: &Operation, img: &mut Image) {
    // Variable declarations
    let size: i32 = img.width.min(img.height);
    let mut freq: f32 = *op.args.as_ref().unwrap()[0].as_float().unwrap();
    let mut water: f32 = *op.args.as_ref().unwrap()[1].as_float().unwrap();

    // Error checking variables
    if (freq < 0.0) {
        eprintln!(
            "Frequency value {} is not > 0 using default value of 1",
            freq
        );
        freq = freq.max(1.0);
    }
    if (!(0.0..=100.0).contains(&water)) {
        eprintln!(
            "Water value {} is not within 0-100, using default value of 50",
            water
        );
        water = water.max(50.0);
    }

    // Running create_random_terrain command
    extra::create_random_terrain(size, freq, water, img);
}
