// Imports
use crate::prev::ver10::format::constant;
use crate::prev::ver10::script::compile::{Lighting, Operation, Symbol};
use std::collections::HashMap;

/// Function that checks that a certain constant exists in the symbol table
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
        return constant::EQV;
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
