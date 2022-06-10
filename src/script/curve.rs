// Imports
use crate::algorithm::curve;
use crate::compiler::{Argument, Operation, Symbol};
use crate::format::{constant, image::Image, matrix::Matrix};
use std::collections::HashMap;

/// Function that performs the 'bezier' command
pub fn bezier(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, cord: &Matrix, img: &mut Image) {
    // Checking for coordinate system
    let mut fnd: bool = true;
    let mut name: String = String::new();
    match op.cs0 {
        None => {
            fnd = false;
        }
        _ => {
            name = op.cs0.as_ref().unwrap().to_string();
        }
    }

    // Getting coordinate system if found and is correct
    let cs: &Matrix = if (fnd) {
        match symbols.get(&name) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name
                );
                cord
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("cs")) {
                    eprintln!("Symbol value {} is not a coordinate system, using default value", name);
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting edge matrix
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut edge: Matrix = Matrix::new_matrix();
    curve::add_bezier(
        &mut edge,
        (*args[0].as_float().unwrap(), *args[1].as_float().unwrap()),
        (*args[2].as_float().unwrap(), *args[3].as_float().unwrap()),
        (*args[4].as_float().unwrap(), *args[5].as_float().unwrap()),
        (*args[6].as_float().unwrap(), *args[7].as_float().unwrap()),
        100,
    );

    // Transforming matrix and adding bezier to image
    edge.left_transform(cs);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'circle' command
pub fn circle(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>,cord: &Matrix, img: &mut Image) {
    // Checking for coordinate system
    let mut fnd: bool = true;
    let mut name: String = String::new();
    match op.cs0 {
        None => {
            fnd = false;
        }
        _ => {
            name = op.cs0.as_ref().unwrap().to_string();
        }
    }

    // Getting coordinate system if found and is correct
    let cs: &Matrix = if (fnd) {
        match symbols.get(&name) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name
                );
                cord
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("cs")) {
                    eprintln!("Symbol value {} is not a coordinate system, using default value", name);
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting edge matrix
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut edge: Matrix = Matrix::new_matrix();
    curve::add_circle(
        &mut edge,
        (
            *args[0].as_float().unwrap(),
            *args[1].as_float().unwrap(),
            *args[2].as_float().unwrap(),
        ),
        *args[3].as_float().unwrap(),
        100,
    );

    // Transforming matrix and adding circle to image
    edge.left_transform(cs);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'hermite' command
pub fn hermite(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>,cord: &Matrix, img: &mut Image) {
    // Checking for coordinate system
    let mut fnd: bool = true;
    let mut name: String = String::new();
    match op.cs0 {
        None => {
            fnd = false;
        }
        _ => {
            name = op.cs0.as_ref().unwrap().to_string();
        }
    }

    // Getting coordinate system if found and is correct
    let cs: &Matrix = if (fnd) {
        match symbols.get(&name) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name
                );
                cord
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("cs")) {
                    eprintln!("Symbol value {} is not a coordinate system, using default value", name);
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting edge matrix
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut edge: Matrix = Matrix::new_matrix();
    curve::add_hermite(
        &mut edge,
        (*args[0].as_float().unwrap(), *args[1].as_float().unwrap()),
        (*args[2].as_float().unwrap(), *args[3].as_float().unwrap()),
        (*args[4].as_float().unwrap(), *args[5].as_float().unwrap()),
        (*args[6].as_float().unwrap(), *args[7].as_float().unwrap()),
        100,
    );

    // Transforming matrix and adding hermite to image
    edge.left_transform(cs);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'line' operation
pub fn line(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>,cord: &Matrix, img: &mut Image) {
    // Checking for first coordinate system
    let mut fnd1: bool = true;
    let mut name1: String = String::new();
    match op.cs0 {
        None => {
            fnd1 = false;
        }
        _ => {
            name1 = op.cs0.as_ref().unwrap().to_string();
        }
    }

    // Getting first coordinate system if found and is correct
    let cs0: &Matrix = if (fnd1) {
        match symbols.get(&name1) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, using default value",
                    name1
                );
                cord
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("cs")) {
                    eprintln!("Symbol value {} is not a coordinate system, using default value", name1);
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };


    // Checking for second coordinate system
    let mut fnd2: bool = true;
    let mut name2: String = String::new();
    match op.cs1 {
        None => {
            fnd2 = false;
        }
        _ => {
            name2 = op.cs1.as_ref().unwrap().to_string();
        }
    }

    // Getting first coordinate system if found and is correct
    let cs1: &Matrix = if (fnd2) {
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

    // Updating points with appropiate coordinate system
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut mat1: Matrix = Matrix::new_matrix();
    mat1.add_point((
        *args[0].as_float().unwrap(),
        *args[1].as_float().unwrap(),
        *args[2].as_float().unwrap(),
    ));
    mat1.left_transform(cs0);
    let mut mat2: Matrix = Matrix::new_matrix();
    mat2.add_point((
        *args[3].as_float().unwrap(),
        *args[4].as_float().unwrap(),
        *args[5].as_float().unwrap(),
    ));
    mat2.left_transform(cs1);


    // Getting edge matrix and drawing line
    let mut edge: Matrix = Matrix::new_matrix();
    edge.append(&mat1);
    edge.append(&mat2);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}
