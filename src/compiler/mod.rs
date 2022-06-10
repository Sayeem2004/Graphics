// Imports
use crate::format::{matrix::Matrix, util};
use crate::script::{parse, parse::ImageInfo};
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, process::Command};

/// Enum to store both strings and floats
#[derive(Serialize, Deserialize, Debug, EnumAsInner)]
#[serde(untagged)]
pub enum Argument {
    Float(f32),
    String(String),
}

/// Struct to store operations
#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub args: Option<Vec<Argument>>,
    pub constants: Option<String>,
    pub cs0: Option<String>,
    pub cs1: Option<String>,
    pub knob: Option<String>,
    pub op: Option<String>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub light: Option<String>,
}

/// Struct to store camera constants
#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
    pub eye: Vec<f32>,
    pub aim: Vec<f32>,
}

/// Struct to store lighting constants
#[derive(Serialize, Deserialize, Debug)]
pub struct Lighting {
    pub red: Vec<f32>,
    pub blue: Vec<f32>,
    pub green: Vec<f32>,
    pub intensity: Option<Vec<f32>>,
}

/// Struct to store light source
#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub color: Vec<f32>,
    pub location: Vec<f32>,
}

/// Enum to store possible symbol values
#[derive(Serialize, Deserialize, Debug, EnumAsInner)]
#[serde(untagged)]
pub enum Symbol {
    Camera(Camera),
    Matrix(Matrix),
    Float(f32),
    Int(i32),
    Lighting(Lighting),
    Source(Source),
    String(String),
    Vec(Vec<f32>),
}

/// Function that compiles a given script file and outputs an image
pub fn compile(path: &str, mode: i32) {
    // Making sure the script file exists
    if (!util::file_exists(path)) {
        eprintln!(
            "Script file {} does not exist, ending compiling attempt",
            path
        );
        return;
    }

    // Running python compiler on script
    let output = Command::new("python")
        .arg("src/compiler/script.py")
        .arg(path)
        .output()
        .expect("Transferring failed");

    // Removing generated python files
    if (util::file_exists("src/compiler/parse.out")) {
        fs::remove_file("src/compiler/parse.out").expect("Unable to delete parse.out file");
    }
    if (util::file_exists("src/compiler/parse.py")) {
        fs::remove_file("src/compiler/parse.py").expect("Unable to delete parse.py file");
    }

    // Checking output from command
    let stdout = String::from_utf8(output.stdout).unwrap();
    if (!stdout.contains("Python: Parsing succeeded")) {
        eprintln!(
            "Script file {} failed to compile, ending compiling attempt\n{}",
            path,
            String::from_utf8(output.stderr).unwrap(),
        );
        return;
    }

    // Getting operation and symbol paths from stdout
    let split = stdout[stdout.find("Python: Parsing succeeded").unwrap()..].split('\n');
    let mut operation_path: &str = "";
    let mut symbol_path: &str = "";
    for (cnt, str) in (0_i32..).zip(split) {
        if (cnt == 1) {
            operation_path = str;
        }
        if (cnt == 2) {
            symbol_path = str;
        }
    }

    // Error checking paths
    if (!util::file_exists(operation_path)) {
        eprintln!(
            "operation file {} does not exist, ending compiling attempt",
            operation_path
        );
        return;
    }
    if (!util::file_exists(symbol_path)) {
        eprintln!(
            "Symbol file {} does not exist, ending compiling attempt",
            symbol_path
        );
        return;
    }

    // Opening files and getting json strings
    let operation_string =
        fs::read_to_string(operation_path).expect("Unable to read operation json file");
    let symbol_string = fs::read_to_string(symbol_path).expect("Unable to read symbol json file");

    // Making operations list and symbol table from json string
    let mut operations: Vec<Operation> = serde_json::from_str(&operation_string).unwrap();
    let mut symbols: HashMap<String, Vec<Symbol>> = serde_json::from_str(&symbol_string).unwrap();

    // // Sending operations list and symbol table to generate function to create output
    generate(&mut operations, &mut symbols, mode);

    // Deleting operation and symbol file
    fs::remove_file(operation_path).expect("Unable to delete temporary command file");
    fs::remove_file(symbol_path).expect("Unable to delete temporary symbol file");
}

pub fn generate(
    operations: &mut Vec<Operation>,
    symbols: &mut HashMap<String, Vec<Symbol>>,
    mode: i32,
) {
    // Running initial parse and getting image/animate info
    let mut info: &mut ImageInfo = &mut parse::initial_parse(operations, symbols);

    // Checking to see if animation is needed or not
    if (info.num_frames > 1) {
        // Error checking basename
        if (info.basename == None) {
            eprintln!("No basename provided in script, using default value");
            info.basename = Some(String::from("default"));
        }

        // Getting knob values from various frames
        let frames: Vec<HashMap<String, f32>> = parse::vary_parse(operations, info);

        // Sending things through animation parser
        info.animate = true;
        parse::animate_parse(operations, symbols, info, frames, mode);
    } else {
        // If no animation is needed just send it through regular parser
        info.animate = false;
        parse::draw_parse(operations, symbols, info, mode);
    }
}
