use crate::template::prev::ver10::format::{image::Image, matrix::Matrix};
use crate::template::prev::ver10::script::{
    compile::{Operation, Symbol},
    curve, light, shape, transform, util,
};
use std::collections::HashMap;

/// Function that parses an operations list and a symbol table and creates an image.
pub fn parse(
    operations: &Vec<Operation>,
    symbols: &HashMap<String, Vec<Symbol>>,
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
                curve::line(&operations[curr], &stack[stack.len() - 1], &mut img);
                curr += 1;
            }

            // Display command
            "display" => {
                util::display(&img, mode);
                curr += 1;
            }

            // Clear command
            "clear" => {
                util::clear(&mut img);
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
                    symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Sphere command
            "sphere" => {
                shape::sphere(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Torus command
            "torus" => {
                shape::torus(
                    &operations[curr],
                    symbols,
                    &stack[stack.len() - 1],
                    &mut img,
                );
                curr += 1;
            }

            // Circle command
            "circle" => {
                curve::circle(&operations[curr], &stack[stack.len() - 1], &mut img);
                curr += 1;
            }

            // Bezier command
            "bezier" => {
                curve::bezier(&operations[curr], &stack[stack.len() - 1], &mut img);
                curr += 1;
            }

            // Hermite command
            "hermite" => {
                curve::hermite(&operations[curr], &stack[stack.len() - 1], &mut img);
                curr += 1;
            }

            // Push command
            "push" => {
                transform::push(&mut stack, &mut sz);
                curr += 1;
            }

            // Pop command
            "pop" => {
                transform::pop(&mut stack, &mut sz);
                curr += 1;
            }

            // Constants command
            "constants" => {
                light::constants(&operations[curr], symbols);
                curr += 1;
            }

            // Camera command
            "camera" => {
                eprintln!("The command \'camera\' is not implemented, no actions performed");
                curr += 1;
            }

            // Light command
            "light" => {
                eprintln!("The command \'light\' is not implemented, no actions performed");
                curr += 1;
            }

            // Mesh command
            "mesh" => {
                eprintln!("The command \'mesh\' is not implemented, no actions performed");
                curr += 1;
            }

            // Set command
            "set" => {
                eprintln!("The command \'set\' is not implemented, no actions performed");
                curr += 1;
            }

            // Basename command
            "basename" => {
                eprintln!("The command \'basename\' is not implemented, no actions performed");
                curr += 1;
            }

            // Save knobs command
            "save_knobs" => {
                eprintln!("The command \'save_knobs\' is not implemented, no actions performed");
                curr += 1;
            }

            // Tween command
            "tween" => {
                eprintln!("The command \'tween\' is not implemented, no actions performed");
                curr += 1;
            }

            // Generate rayfiles command
            "generate_rayfiles" => {
                eprintln!(
                    "The command \'generate_rayfiles\' is not implemented, no actions performed"
                );
                curr += 1;
            }

            // Shading command
            "shading" => {
                eprintln!("The command \'shading\' is not implemented, no actions performed");
                curr += 1;
            }

            // Save coord system command
            "save_coord_system" => {
                eprintln!(
                    "The command \'save_coord_system\' is not implemented, no actions performed"
                );
                curr += 1;
            }

            // Focal command
            "focal" => {
                eprintln!("The command \'focal\' is not implemented, no actions performed");
                curr += 1;
            }

            // Set knobs command
            "setknobs" => {
                eprintln!("The command \'setknobs\' is not implemented, no actions performed");
                curr += 1;
            }

            // Ambient command
            "ambient" => {
                eprintln!("The command \'ambient\' is not implemented, no actions performed");
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
