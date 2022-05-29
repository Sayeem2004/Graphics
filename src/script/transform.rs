// Imports
use crate::compiler::{Argument, Operation, Symbol};
use crate::format::matrix::Matrix;
use std::collections::HashMap;

/// Function that performs the 'move' command
pub fn _move(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, cord: &mut Matrix) {
    // Getting number arguments
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut x = *args[0].as_float().unwrap();
    let mut y = *args[1].as_float().unwrap();
    let mut z = *args[2].as_float().unwrap();

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
                    let scale = *list[1].as_float().unwrap();
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
    let mut ang = *args[1].as_float().unwrap();

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
                    let scale = *list[1].as_float().unwrap();
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

/// Function that performs the 'scale' command
pub fn scale(op: &Operation, symbols: &HashMap<String, Vec<Symbol>>, cord: &mut Matrix) {
    // Getting number arguments
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut x = *args[0].as_float().unwrap();
    let mut y = *args[1].as_float().unwrap();
    let mut z = *args[2].as_float().unwrap();

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
                    let scale = *list[1].as_float().unwrap();
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
