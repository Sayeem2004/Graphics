// Imports
use crate::compiler::{Argument, Lighting, Operation, Source, Symbol};
use crate::format::{constant, pixel::Pixel};
use std::collections::HashMap;

/// Function that performs the 'alterlight' command
pub fn alterlight(op: &Operation, symbols: &mut HashMap<String, Vec<Symbol>>) {
    // Getting number arguments
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut r: f32 = *args[0].as_float().unwrap();
    let mut g: f32 = *args[1].as_float().unwrap();
    let mut b: f32 = *args[2].as_float().unwrap();

    // Checking for light
    let name: &String = op.light.as_ref().unwrap();
    if (!symbols.contains_key(name)) {
        eprintln!(
            "Light {} not found in the symbol table, no changes made",
            name
        );
        return;
    }
    let mut color: Vec<f32> = symbols.get(name).unwrap()[1]
        .as_source()
        .unwrap()
        .color
        .clone();
    let location: Vec<f32> = symbols.get(name).unwrap()[1]
        .as_source()
        .unwrap()
        .location
        .clone();

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
    (r, g, b) = if (fnd) {
        match symbols.get(&knob) {
            None => {
                eprintln!(
                    "Symbol {} could not be found in symbol table, no changes made",
                    knob
                );
                (r, g, b)
            }
            Some(list) => {
                if (!list[0].as_string().unwrap().eq("knob")) {
                    eprintln!("Symbol value {} is not a knob, no changes made", knob);
                    (r, g, b)
                } else {
                    let scale: f32 = *list[1].as_float().unwrap();
                    (r * scale, g * scale, b * scale)
                }
            }
        }
    } else {
        (r, g, b)
    };

    // Updating light value
    color[0] += r;
    color[1] += g;
    color[2] += b;
    symbols.insert(
        (*name).clone(),
        vec![
            Symbol::String(String::from("light")),
            Symbol::Source(Source { color, location }),
        ],
    );
}

/// Function that checks the 'ambient' command
pub fn ambient(symbols: &mut HashMap<String, Vec<Symbol>>) {
    // Checking if it is inside the symbol table
    if (!symbols.contains_key("ambient")) {
        eprintln!("Ambient light not found in the symbol table");
    }
}

/// Function that checks the 'constants' command
pub fn constants(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>) {
    // Getting constants name
    let name: &String = op.constants.as_ref().unwrap();

    // Checking if it is inside the symbol table
    if (!symbols.contains_key(name)) {
        eprintln!("Constant {} not found in the symbol table", name);
    }
}

/// Function that converts lighting constants struct into a tuple
pub fn constants_to_tuple(constants: &[Symbol]) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    // Error checking
    let typ: &String = constants[0].as_string().unwrap();
    if (!typ.eq("constants")) {
        eprintln!("Symbol table element type is not constants, returning default value");
        return constant::EQVDIV;
    }
    if (constants.len() <= 1) {
        eprintln!(
            "Symbol table element does not have a lighting structure, returning default value"
        );
        return constant::EQVDIV;
    }

    // Making and returning tuple of elements
    let lighting: &Lighting = constants[1].as_lighting().unwrap();
    (
        lighting.red[0],
        lighting.green[0],
        lighting.blue[0],
        lighting.red[1],
        lighting.green[1],
        lighting.blue[1],
        lighting.red[2],
        lighting.green[2],
        lighting.blue[2],
    )
}

/// Function that performs the 'light' command
pub fn light(op: &Operation, symbols: &mut HashMap<String, Vec<Symbol>>, lights: &mut Vec<String>) {
    // Variable declarations
    let name: String = (*op.light.as_ref().unwrap()).clone();
    let r: f32 = *op.args.as_ref().unwrap()[0].as_float().unwrap();
    let g: f32 = *op.args.as_ref().unwrap()[1].as_float().unwrap();
    let b: f32 = *op.args.as_ref().unwrap()[2].as_float().unwrap();
    let x: f32 = *op.args.as_ref().unwrap()[3].as_float().unwrap();
    let y: f32 = *op.args.as_ref().unwrap()[4].as_float().unwrap();
    let z: f32 = *op.args.as_ref().unwrap()[5].as_float().unwrap();

    // Adding to symbol table and lights list
    lights.push((name).clone());
    symbols.insert(
        name,
        vec![
            Symbol::String(String::from("light")),
            Symbol::Source(Source {
                color: vec![r, g, b],
                location: vec![x, y, z],
            }),
        ],
    );
}

/// Function that performs the 'movelight' command
pub fn movelight(op: &Operation, symbols: &mut HashMap<String, Vec<Symbol>>) {
    // Getting number arguments
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut x: f32 = *args[0].as_float().unwrap();
    let mut y: f32 = *args[1].as_float().unwrap();
    let mut z: f32 = *args[2].as_float().unwrap();

    // Checking for light
    let name: &String = op.light.as_ref().unwrap();
    if (!symbols.contains_key(name)) {
        eprintln!(
            "Light {} not found in the symbol table, no changes made",
            name
        );
        return;
    }
    let color: Vec<f32> = symbols.get(name).unwrap()[1]
        .as_source()
        .unwrap()
        .color
        .clone();
    let mut location: Vec<f32> = symbols.get(name).unwrap()[1]
        .as_source()
        .unwrap()
        .location
        .clone();

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

    // Updating light value
    location[0] += x;
    location[1] += y;
    location[2] += z;
    symbols.insert(
        (*name).clone(),
        vec![
            Symbol::String(String::from("light")),
            Symbol::Source(Source { color, location }),
        ],
    );
}

/// Function that converts from a list of light names to a list of light values and positions
pub fn names_to_sources(
    lights: &[String],
    symbols: &HashMap<String, Vec<Symbol>>,
) -> Vec<(Pixel, f32, f32, f32)> {
    // Variable declarations
    let mut ret: Vec<(Pixel, f32, f32, f32)> = Vec::new();

    // Iterating through light names
    for name in lights.iter() {
        let light: &Source = symbols.get(name).unwrap()[1].as_source().unwrap();
        ret.push((
            Pixel::new_value(
                light.color[0].min(255.0).max(0.0) as u8,
                light.color[1].min(255.0).max(0.0) as u8,
                light.color[2].min(255.0).max(0.0) as u8,
            ),
            light.location[0],
            light.location[1],
            light.location[2],
        ));
    }

    // Exiting function
    ret
}
