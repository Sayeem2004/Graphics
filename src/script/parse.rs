use crate::format::{image::Image, matrix::Matrix};
use crate::script::{
    compile::{Operation, Symbol},
    shape, transform, util,
};
use std::collections::HashMap;

/// Function that parses an operations list and a symbol table and creates an image.
pub fn parse(
    operations: Vec<Operation>,
    symbols: HashMap<String, Vec<Symbol>>,
    size: i32,
    mode: i32,
) {
    // Indexing variables
    let mx: usize = operations.len();
    let mut curr: usize = 0;

    // Creating coordinate stack and image struct
    let mut stack: Vec<Matrix> = vec![Matrix::new_transformation()];
    let mut sz: usize = stack.len();
    let mut img: Image = Image::new_dimension(size, size);

    // Looping through all lines
    while (curr < mx) {
        match operations[curr].op.as_ref().unwrap().as_str() {
            // Line command
            "line" => {
                util::line(&operations[curr], &stack[stack.len() - 1], &mut img);
                curr += 1;
            }

            // Display command
            "display" => {
                util::display(&img, mode);
                curr += 1;
            }

            // Scale command
            "scale" => {
                transform::scale(&operations[curr], &mut stack[sz - 1]);
                curr += 1;
            }

            // Move command
            "move" => {
                transform::_move(&operations[curr], &mut stack[sz - 1]);
                curr += 1;
            }

            // Rotate command
            "rotate" => {
                transform::rotate(&operations[curr], &mut stack[sz - 1]);
                curr += 1;
            }

            // Save command
            "save" => {
                util::save(&operations[curr], &img);
                curr += 1;
            }

            // Box command
            "box" => {
                shape::_box(
                    &operations[curr],
                    &symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Sphere command
            "sphere" => {
                shape::sphere(
                    &operations[curr],
                    &symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Torus command
            "torus" => {
                shape::torus(
                    &operations[curr],
                    &symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Push command
            "push" => {
                util::push(&mut stack, &mut sz);
                curr += 1;
            }

            // Pop command
            "pop" => {
                util::pop(&mut stack, &mut sz);
                curr += 1;
            }

            // Constants command
            "constants" => {
                util::constants(&operations[curr], &symbols);
                curr += 1;
            }

            // Default case
            _ => {
                eprintln!(
                    "The command \'{}\' is invalid",
                    operations[curr].op.as_ref().unwrap().as_str()
                );
                curr += 1;
            }
        }
    }
}
