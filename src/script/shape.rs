// Imports
use crate::algorithm::shape;
use crate::compiler::{Operation, Symbol};
use crate::format::{constant, image::Image, matrix::Matrix, pixel::Pixel};
use crate::script::light;
use std::collections::HashMap;

/// Function that performs the 'box' command
pub fn _box(
    op: &Operation,
    symbols: &HashMap<String, Vec<Symbol>>,
    cord: &Matrix,
    img: &mut Image,
) {
    // Checking for lighting constants
    let mut fnd1: bool = true;
    let mut name1: String = String::new();
    match op.constants {
        None => {
            fnd1 = false;
        }
        _ => {
            name1 = op.constants.as_ref().unwrap().to_string();
        }
    }

    // Getting lighting constants if found and is correct
    let constants: (f32, f32, f32, f32, f32, f32, f32, f32, f32) = if (fnd1) {
        match symbols.get(&name1) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name1
                );
                constant::EQV
            }
            Some(list) => light::constants_to_tuple(list),
        }
    } else {
        constant::EQV
    };

    // Checking for coordinate system
    let mut fnd2: bool = true;
    let mut name2: String = String::new();
    match op.cs0 {
        None => {
            fnd2 = false;
        }
        _ => {
            name2 = op.cs0.as_ref().unwrap().to_string();
        }
    }

    // Getting coordinate system if found and is correct
    let cs: &Matrix = if (fnd2) {
        match symbols.get(&name2) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name2
                );
                cord
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("cs")) {
                    eprintln!("Symbol value {} is not a coordinate system, using default value", name2);
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting box coordinates and adding it to edge matrix
    let args = op.args.as_ref().unwrap();
    let mut poly: Matrix = Matrix::new_matrix();
    shape::add_box(
        &mut poly,
        (
            *args[0].as_float().unwrap(),
            *args[1].as_float().unwrap(),
            *args[2].as_float().unwrap(),
        ),
        *args[4].as_float().unwrap(),
        *args[3].as_float().unwrap(),
        *args[5].as_float().unwrap(),
    );

    // Adding box to image
    poly.left_transform(cs);
    poly.draw_triangles_xy(
        img,
        Pixel::new_scale(0.5),
        &[(Pixel::new_scale(1.0), 750.0, 750.0, 750.0)],
        constant::ZVIEW,
        constants,
    );
}

/// Function that performs the 'sphere' command
pub fn sphere(
    op: &Operation,
    symbols: &HashMap<String, Vec<Symbol>>,
    cord: &Matrix,
    img: &mut Image,
) {
    // Checking for lighting constants
    let mut fnd1: bool = true;
    let mut name1: String = String::new();
    match op.constants {
        None => {
            fnd1 = false;
        }
        _ => {
            name1 = op.constants.as_ref().unwrap().to_string();
        }
    }

    // Getting lighting constants if found and is correct
    let constants: (f32, f32, f32, f32, f32, f32, f32, f32, f32) = if (fnd1) {
        match symbols.get(&name1) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name1
                );
                constant::EQV
            }
            Some(list) => light::constants_to_tuple(list),
        }
    } else {
        constant::EQV
    };

    // Checking for coordinate system
    let mut fnd2: bool = true;
    let mut name2: String = String::new();
    match op.cs0 {
        None => {
            fnd2 = false;
        }
        _ => {
            name2 = op.cs0.as_ref().unwrap().to_string();
        }
    }

    // Getting coordinate system if found and is correct
    let cs: &Matrix = if (fnd2) {
        match symbols.get(&name2) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name2
                );
                cord
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("cs")) {
                    eprintln!("Symbol value {} is not a coordinate system, using default value", name2);
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting sphere coordinates and adding it to edge matrix
    let args = op.args.as_ref().unwrap();
    let mut poly: Matrix = Matrix::new_matrix();
    shape::add_sphere(
        &mut poly,
        (
            *args[0].as_float().unwrap(),
            *args[1].as_float().unwrap(),
            *args[2].as_float().unwrap(),
        ),
        *args[3].as_float().unwrap(),
        100,
    );

    // Adding sphere to image
    poly.left_transform(cs);
    poly.draw_triangles_xy(
        img,
        Pixel::new_scale(0.5),
        &[(Pixel::new_scale(1.0), 750.0, 750.0, 750.0)],
        constant::ZVIEW,
        constants,
    );
}

/// Function that performs the 'torus' command
pub fn torus(
    op: &Operation,
    symbols: &HashMap<String, Vec<Symbol>>,
    cord: &Matrix,
    img: &mut Image,
) {
    // Checking for lighting constants
    let mut fnd1: bool = true;
    let mut name1: String = String::new();
    match op.constants {
        None => {
            fnd1 = false;
        }
        _ => {
            name1 = op.constants.as_ref().unwrap().to_string();
        }
    }

    // Getting lighting constants if found and is correct
    let constants: (f32, f32, f32, f32, f32, f32, f32, f32, f32) = if (fnd1) {
        match symbols.get(&name1) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name1
                );
                constant::EQV
            }
            Some(list) => light::constants_to_tuple(list),
        }
    } else {
        constant::EQV
    };

    // Checking for coordinate system
    let mut fnd2: bool = true;
    let mut name2: String = String::new();
    match op.cs0 {
        None => {
            fnd2 = false;
        }
        _ => {
            name2 = op.cs0.as_ref().unwrap().to_string();
        }
    }

    // Getting coordinate system if found and is correct
    let cs: &Matrix = if (fnd2) {
        match symbols.get(&name2) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name2
                );
                cord
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("cs")) {
                    eprintln!("Symbol value {} is not a coordinate system, using default value", name2);
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting torus coordinates and adding it to edge matrix
    let args = op.args.as_ref().unwrap();
    let mut poly: Matrix = Matrix::new_matrix();
    shape::add_torus(
        &mut poly,
        (
            *args[0].as_float().unwrap(),
            *args[1].as_float().unwrap(),
            *args[2].as_float().unwrap(),
        ),
        *args[3].as_float().unwrap(),
        *args[4].as_float().unwrap(),
        100,
    );

    // Adding torus to image
    poly.left_transform(cs);
    poly.draw_triangles_xy(
        img,
        Pixel::new_scale(0.5),
        &[(Pixel::new_scale(1.0), 750.0, 750.0, 750.0)],
        constant::ZVIEW,
        constants,
    );
}
