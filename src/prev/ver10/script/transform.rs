// Imports
use crate::prev::ver10::format::matrix::Matrix;
use crate::prev::ver10::script::compile::{Argument, Operation};

/// Function that performs the 'move' command
pub fn _move(op: &Operation, cord: &mut Matrix) {
    // Getting translation matrix
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let trans: Matrix = Matrix::translate(
        *args[0].as_float().unwrap(),
        *args[1].as_float().unwrap(),
        *args[2].as_float().unwrap(),
    );

    // Performing translation
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
pub fn rotate(op: &Operation, cord: &mut Matrix) {
    // Getting rotation matrix
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let trans: Matrix =
        Matrix::rotate_degree(*args[1].as_float().unwrap(), args[0].as_string().unwrap());

    // Performing rotation
    cord.right_transform(&trans);
}

/// Function that performs the 'scale' command
pub fn scale(op: &Operation, cord: &mut Matrix) {
    // Getting rotation matrix
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let trans: Matrix = Matrix::dilate(
        *args[0].as_float().unwrap(),
        *args[1].as_float().unwrap(),
        *args[2].as_float().unwrap(),
    );

    // Performing dilation
    cord.right_transform(&trans);
}
