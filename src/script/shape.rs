// Imports
use crate::algorithm::shape;
use crate::compiler::{Operation, Symbol};
use crate::format::{constant, image::Image, matrix::Matrix, pixel::Pixel};
use crate::script::{light, parse::ImageInfo};
use std::collections::HashMap;

/// Function that performs the 'box' command
pub fn _box(
    op: &Operation,
    symbols: &HashMap<String, Vec<Symbol>>,
    cord: &Matrix,
    info: &ImageInfo,
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
                constant::EQVDIV
            }
            Some(list) => light::constants_to_tuple(list),
        }
    } else {
        constant::EQVDIV
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
                    eprintln!(
                        "Symbol value {} is not a coordinate system, using default value",
                        name2
                    );
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting ambient light if found and is correct
    let ambient: Pixel = match symbols.get("ambient") {
        None => *constant::AMB,
        Some(list) => {
            if (!list[0].as_string().unwrap().eq("ambient")) {
                eprintln!("Symbol value ambient is not an ambient light, using default value");
                *constant::AMB
            } else {
                let r = *list[1].as_float().unwrap();
                let g = *list[2].as_float().unwrap();
                let b = *list[3].as_float().unwrap();
                Pixel::new_value(r.min(255.0) as u8, g.min(255.0) as u8, b.min(255.0) as u8)
            }
        }
    };

    // Getting point light sources if they exist
    let pnts: Vec<(Pixel, f32, f32, f32)> = if (info.points.is_empty()) {
        (*constant::PNTS).clone()
    } else {
        light::names_to_sources(info, symbols)
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
    poly.draw_triangles_xy(img, ambient, &pnts, constant::ZVIEW, constants);
}

/// Function that performs the 'sphere' command
pub fn sphere(
    op: &Operation,
    symbols: &HashMap<String, Vec<Symbol>>,
    cord: &Matrix,
    info: &ImageInfo,
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
                constant::EQVDIV
            }
            Some(list) => light::constants_to_tuple(list),
        }
    } else {
        constant::EQVDIV
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
                    eprintln!(
                        "Symbol value {} is not a coordinate system, using default value",
                        name2
                    );
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting ambient light if found and is correct
    let ambient: Pixel = match symbols.get("ambient") {
        None => *constant::AMB,
        Some(list) => {
            if (!list[0].as_string().unwrap().eq("ambient")) {
                eprintln!("Symbol value ambient is not an ambient light, using default value");
                *constant::AMB
            } else {
                let r = *list[1].as_float().unwrap();
                let g = *list[2].as_float().unwrap();
                let b = *list[3].as_float().unwrap();
                Pixel::new_value(r.min(255.0) as u8, g.min(255.0) as u8, b.min(255.0) as u8)
            }
        }
    };

    // Getting point light sources if they exist
    let pnts: Vec<(Pixel, f32, f32, f32)> = if (info.points.is_empty()) {
        (*constant::PNTS).clone()
    } else {
        light::names_to_sources(info, symbols)
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
    poly.draw_triangles_xy(img, ambient, &pnts, constant::ZVIEW, constants);
}

/// Function that performs the 'torus' command
pub fn torus(
    op: &Operation,
    symbols: &HashMap<String, Vec<Symbol>>,
    cord: &Matrix,
    info: &ImageInfo,
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
                constant::EQVDIV
            }
            Some(list) => light::constants_to_tuple(list),
        }
    } else {
        constant::EQVDIV
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
                    eprintln!(
                        "Symbol value {} is not a coordinate system, using default value",
                        name2
                    );
                    cord
                } else {
                    list[1].as_matrix().unwrap()
                }
            }
        }
    } else {
        cord
    };

    // Getting ambient light if found and is correct
    let ambient: Pixel = match symbols.get("ambient") {
        None => *constant::AMB,
        Some(list) => {
            if (!list[0].as_string().unwrap().eq("ambient")) {
                eprintln!("Symbol value ambient is not an ambient light, using default value");
                *constant::AMB
            } else {
                let r = *list[1].as_float().unwrap();
                let g = *list[2].as_float().unwrap();
                let b = *list[3].as_float().unwrap();
                Pixel::new_value(
                    r.min(255.0).max(0.0) as u8,
                    g.min(255.0).max(0.0) as u8,
                    b.min(255.0).max(0.0) as u8,
                )
            }
        }
    };

    // Getting point light sources if they exist
    let pnts: Vec<(Pixel, f32, f32, f32)> = if (info.points.is_empty()) {
        (*constant::PNTS).clone()
    } else {
        light::names_to_sources(info, symbols)
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
    poly.draw_triangles_xy(img, ambient, &pnts, constant::ZVIEW, constants);
}
