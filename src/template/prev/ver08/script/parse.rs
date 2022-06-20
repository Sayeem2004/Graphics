// Imports
use crate::template::prev::ver08::format::{file, image::Image, matrix::Matrix};
use crate::template::prev::ver08::script::{curve, shape, transform, util};

/// Function that parses a graphics script file and runs commands
pub fn parse(path: &str, size: i32, mode: i32) {
    // Making sure the script file exists
    if (!file::file_exists(path)) {
        eprintln!("Script file does not exist, ending parsing attempt");
        return;
    }

    // Getting all lines from the script file
    let lines: Vec<String> = file::read_lines(path);
    let mx: usize = lines.len();
    let mut curr: usize = 0;

    // Creating coordinate stack, edge matrix, polygon matrix, and image struct
    let mut stack: Vec<Matrix> = vec![Matrix::new_transformation()];
    let mut sz: usize = stack.len();
    let mut img: Image = Image::new_dimension(size, size);

    // Looping through all lines
    while (curr < mx) {
        match lines[curr].as_str() {
            // Line command
            "line" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'line\' at line {}",
                        curr + 1
                    );
                } else {
                    util::line(
                        &lines[curr + 1],
                        curr + 1,
                        &stack[stack.len() - 1],
                        &mut img,
                    );
                }
                curr += 2;
            }

            // Display command
            "display" => {
                if (mode == 0) {
                    util::display(&img);
                }
                curr += 1;
            }

            // Scale command
            "scale" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'scale\' at line {}",
                        curr + 1
                    );
                } else {
                    transform::scale(&lines[curr + 1], curr + 1, &mut stack[sz - 1]);
                }
                curr += 2;
            }

            // Move command
            "move" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'move\' at line {}",
                        curr + 1
                    );
                } else {
                    transform::_move(&lines[curr + 1], curr + 1, &mut stack[sz - 1]);
                }
                curr += 2;
            }

            // Rotate command
            "rotate" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'rotate\' at line {}",
                        curr + 1
                    );
                } else {
                    transform::rotate(&lines[curr + 1], curr + 1, &mut stack[sz - 1]);
                }
                curr += 2;
            }

            // Save command
            "save" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'save\' at line {}",
                        curr + 1
                    );
                } else {
                    util::save(&lines[curr + 1], &img);
                }
                curr += 2;
            }

            // Circle command
            "circle" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'circle\' at line {}",
                        curr + 1
                    );
                } else {
                    curve::circle(
                        &lines[curr + 1],
                        curr + 1,
                        &stack[stack.len() - 1],
                        &mut img,
                    );
                }
                curr += 2;
            }

            // Hermite command
            "hermite" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'hermite\' at line {}",
                        curr + 1
                    );
                } else {
                    curve::hermite(
                        &lines[curr + 1],
                        curr + 1,
                        &stack[stack.len() - 1],
                        &mut img,
                    );
                }
                curr += 2;
            }

            // Bezier command
            "bezier" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'bezier\' at line {}",
                        curr + 1
                    );
                } else {
                    curve::bezier(
                        &lines[curr + 1],
                        curr + 1,
                        &stack[stack.len() - 1],
                        &mut img,
                    );
                }
                curr += 2;
            }

            // Box command
            "box" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'box\' at line {}",
                        curr + 1
                    );
                } else {
                    shape::_box(
                        &lines[curr + 1],
                        curr + 1,
                        &stack[stack.len() - 1],
                        &mut img,
                    );
                }
                curr += 2;
            }

            // Sphere command
            "sphere" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'sphere\' at line {}",
                        curr + 1
                    );
                } else {
                    shape::sphere(
                        &lines[curr + 1],
                        curr + 1,
                        &stack[stack.len() - 1],
                        &mut img,
                    );
                }
                curr += 2;
            }

            // Torus command
            "torus" => {
                if (curr == mx - 1) {
                    eprintln!(
                        "No arguments provided for command \'torus\' at line {}",
                        curr + 1
                    );
                } else {
                    shape::torus(
                        &lines[curr + 1],
                        curr + 1,
                        &stack[stack.len() - 1],
                        &mut img,
                    );
                }
                curr += 2;
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

            // Clear command
            "clear" => {
                util::clear(&mut img);
                curr += 1;
            }

            // Empty line case
            "" => curr += 1,

            // Default case
            _ => {
                if (!lines[curr][0..1].eq("#")) {
                    eprintln!(
                        "The command \'{}\' found at line {} is invalid",
                        lines[curr],
                        curr + 1
                    );
                }
                curr += 1;
            }
        }
    }
}
