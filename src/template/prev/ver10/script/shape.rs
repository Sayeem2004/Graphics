// Imports
use crate::template::prev::ver10::algorithm::shape;
use crate::template::prev::ver10::format::{constant, image::Image, matrix::Matrix, pixel::Pixel};
use crate::template::prev::ver10::script::{
    compile::{Operation, Symbol},
    light,
};
use std::collections::HashMap;

/// Function that performs the 'box' command
pub fn _box(
    op: &Operation,
    symbols: &HashMap<String, Vec<Symbol>>,
    cord: &Matrix,
    img: &mut Image,
) {
    // Checking for lighting constants
    let mut fnd: bool = true;
    let mut name: String = String::new();
    match op.constants {
        None => {
            fnd = false;
        }
        _ => {
            name = op.constants.as_ref().unwrap().to_string();
        }
    }

    // Getting lighting constants if found and is correct
    let constants: (f32, f32, f32, f32, f32, f32, f32, f32, f32) = if (fnd) {
        match symbols.get(&name) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name
                );
                constant::EQV
            }
            Some(_) => light::constants_to_tuple(symbols.get(&name).unwrap()),
        }
    } else {
        constant::EQV
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
    poly.left_transform(cord);
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
    let mut fnd: bool = true;
    let mut name: String = String::new();
    match op.constants {
        None => {
            fnd = false;
        }
        _ => {
            name = op.constants.as_ref().unwrap().to_string();
        }
    }

    // Getting lighting constants if found and is correct
    let constants: (f32, f32, f32, f32, f32, f32, f32, f32, f32) = if (fnd) {
        match symbols.get(&name) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name
                );
                constant::EQV
            }
            Some(_) => light::constants_to_tuple(symbols.get(&name).unwrap()),
        }
    } else {
        constant::EQV
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
    poly.left_transform(cord);
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
    let mut fnd: bool = true;
    let mut name: String = String::new();
    match op.constants {
        None => {
            fnd = false;
        }
        _ => {
            name = op.constants.as_ref().unwrap().to_string();
        }
    }

    // Getting lighting constants if found and is correct
    let constants: (f32, f32, f32, f32, f32, f32, f32, f32, f32) = if (fnd) {
        match symbols.get(&name) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name
                );
                constant::EQV
            }
            Some(_) => light::constants_to_tuple(symbols.get(&name).unwrap()),
        }
    } else {
        constant::EQV
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
    poly.left_transform(cord);
    poly.draw_triangles_xy(
        img,
        Pixel::new_scale(0.5),
        &[(Pixel::new_scale(1.0), 750.0, 750.0, 750.0)],
        constant::ZVIEW,
        constants,
    );
}
