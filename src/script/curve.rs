// Imports
use crate::algorithm::curve;
use crate::format::{constant, image::Image, matrix::Matrix};
use crate::script::compile::{Argument, Operation};

/// Function that performs the 'bezier' command
pub fn bezier(op: &Operation, cord: &Matrix, img: &mut Image) {
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
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'circle' command
pub fn circle(op: &Operation, cord: &Matrix, img: &mut Image) {
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
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'hermite' command
pub fn hermite(op: &Operation, cord: &Matrix, img: &mut Image) {
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
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}

/// Function that performs the 'line' operation
pub fn line(op: &Operation, cord: &Matrix, img: &mut Image) {
    // Getting edge matrix
    let args: &Vec<Argument> = op.args.as_ref().unwrap();
    let mut edge: Matrix = Matrix::new_matrix();
    edge.add_edge(
        (
            *args[0].as_float().unwrap(),
            *args[1].as_float().unwrap(),
            *args[2].as_float().unwrap(),
        ),
        (
            *args[3].as_float().unwrap(),
            *args[4].as_float().unwrap(),
            *args[5].as_float().unwrap(),
        ),
    );

    // Transforming matrix and drawing line
    edge.left_transform(cord);
    edge.draw_lines_xy(img, constant::WHITE_PIXEL);
}
