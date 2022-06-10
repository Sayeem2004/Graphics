// Imports
use crate::compiler::{Lighting, Operation, Symbol};
use crate::format::{constant, pixel::Pixel};
use crate::script::parse::ImageInfo;
use std::collections::HashMap;

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
    let typ = constants[0].as_string().unwrap();
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
pub fn light(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, info: &mut ImageInfo) {
    // Getting constants name
    let name: &String = op.light.as_ref().unwrap();

    // Checking if it is inside the symbol table
    if (!symbols.contains_key(name)) {
        eprintln!(
            "Light {} not found in the symbol table, not including it within light list",
            name
        );
    } else {
        info.points.push((*name).clone());
    }
}

/// Function that converts from a list of light names to a list of light values and positions
pub fn names_to_sources(
    info: &ImageInfo,
    symbols: &HashMap<String, Vec<Symbol>>,
) -> Vec<(Pixel, f32, f32, f32)> {
    // Variable declarations
    let mut ret: Vec<(Pixel, f32, f32, f32)> = Vec::new();

    // Iterating through light names
    for name in info.points.iter() {
        let light = symbols.get(name).unwrap()[1].as_source().unwrap();
        ret.push((
            Pixel::new_value(
                light.color[0].min(255.0).max(0.0) as u8,
                light.color[1].min(255.0).max(0.0) as u8,
                light.color[2].min(255.0).max(0.0) as u8,
            ),
            light.location[0],
            light.location[1],
            light.location[2],
        ))
    }

    // Exiting function
    ret
}
