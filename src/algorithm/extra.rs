// Imports
use crate::format::{image::Image, matrix::Matrix, pixel::Pixel, util};
use crate::io::Error;
use std::{
    fs,
    process::{Command, Output},
    str::Split,
};

/// Function that sends data to extra.py and generate random data using perlin noise
pub fn create_random_terrain(size: i32, freq: f32, water: f32, img: &mut Image) {
    // Running python generator on arguments
    let output: Output = Command::new("python")
        .arg("src/algorithm/extra.py")
        .arg("create_random_terrain")
        .arg(size.to_string())
        .arg(freq.to_string())
        .arg(water.to_string())
        .output()
        .expect("Transferring failed");

    // Checking output from command
    let stdout: String = String::from_utf8(output.stdout).unwrap();
    if (!stdout.contains("Python: Generating succeeded")) {
        eprintln!(
            "Data file temp/data.csv failed to be generated, ending terrain attempt\n{}",
            String::from_utf8(output.stderr).unwrap(),
        );
        return;
    }

    // Getting data path from stdout
    let split: Split<char> =
        stdout[stdout.find("Python: Generating succeeded").unwrap()..].split('\n');
    let mut csv_path: &str = "";
    for (cnt, str) in (0_i32..).zip(split) {
        if (cnt == 1) {
            csv_path = str;
        }
    }

    // Error checking paths
    if (!util::file_exists(csv_path)) {
        eprintln!(
            "Data file {} does not exist, ending generating attempt",
            csv_path
        );
        return;
    }

    // Reading in data
    let result: Result<String, Error> = fs::read_to_string(csv_path);
    match result {
        Err(_) => {
            eprintln!(
                "Could not read from data file {}, ending generating attempt",
                csv_path
            );
            return;
        }

        Ok(csv_string) => {
            let nums: &Vec<f32> = &csv_string
                .split('\n')
                .map(|x| x.parse::<f32>().unwrap_or(0.0))
                .collect();
            let (mut mn, mut mx): (f32, f32) = (0.0, 0.0);
            for num in nums {
                mx = mx.max(*num);
                mn = mn.max(*num);
            }
            for i in 0..size {
                for q in 0..size {
                    let val: f32 = nums[(i * size + q) as usize];
                    let pix: Pixel = gradient(val, if (val < 0.0) { mn.abs() } else { mx.abs() });
                    img.update_pixel_xy(i, q, val, pix);
                }
            }
        }
    }

    // Removing csv file if it exists
    if (util::file_exists(csv_path)) {
        fs::remove_file(csv_path).expect("Unable to delete temporary data file");
    }
}

/// Function that returns a color value based on the altitude and maximum inputted
pub fn gradient(alt: f32, max: f32) -> Pixel {
    if (alt < 0.0) {
        Pixel::new_value(
            0,
            0,
            (255.0 + (alt / max) * 128.0).max(0.0).min(255.0) as u8,
        )
    } else {
        Pixel::new_value(
            0,
            (128.0 + (alt / max) * 128.0).max(0.0).min(255.0) as u8,
            0,
        )
    }
}

/// Function that parses a mesh file and creates a poly matrix
pub fn parse_mesh(path: &String) -> Matrix {
    // Variable declarations
    let mut full_path = String::from("data/obj/");
    full_path.push_str(path);
    let mut poly = Matrix::new_matrix();
    let mut vertex: Vec<(f32, f32, f32)> = Vec::new();
    let mut normal: Vec<(f32, f32, f32)> = Vec::new();
    let mut texture: Vec<(f32, f32, f32)> = Vec::new();

    // Getting data string and matching it
    let result: Result<String, Error> = fs::read_to_string(&full_path);
    match result {
        // Failure case
        Err(_) => {
            eprintln!("Could not read from obj file {}, using default value", path);
            return poly;
        }

        // Working case
        Ok(data_string) => {
            let lines: Vec<String> = data_string.split('\n').map(|s| s.to_owned()).collect();
            for line in lines.iter() {
                // Blank line or comment
                if (line.eq("") || line[0..1].eq("#")) {
                    continue;
                }

                // Vertex Normal
                if (line[0..2].eq("vn")) {
                    normal.push(parse_three_float(line));
                    continue;
                }

                // Vertex Texture
                if (line[0..2].eq("vt")) {
                    texture.push(parse_three_float(line));
                    continue;
                }

                // Vertex
                if (line[0..1].eq("v")) {
                    vertex.push(parse_three_float(line));
                    continue;
                }
            }

            for line in lines.iter() {
                // Blank line or comment
                if (line.eq("") || line[0..1].eq("#")) {
                    continue;
                }

                // Face
                if (line[0..1].eq("f")) {
                    poly.append(&parse_face(line, &vertex));
                }
            }
        }
    }

    // Exiting function
    poly
}

/// Function that parses a face line
pub fn parse_face(line: &str, vertex: &[(f32, f32, f32)]) -> Matrix {
    // Variable declarations
    let mut poly: Matrix = Matrix::new_matrix();
    let mut list: Vec<(f32, f32, f32)> = Vec::new();

    // Iterating through split line
    for (i, str) in (0_i32..).zip(line.split(' ')) {
        if (!str.is_empty() && i > 0) {
            // Iterating through split indices
            for (q, ind) in (0_i32..).zip(str.split('/')) {
                if (q == 0) {
                    let (x, y, z) = vertex[ind.parse::<usize>().unwrap_or(0) - 1_usize];
                    list.push((x, y, z));
                }
            }
        }
    }

    // Dealing with the cw order
    list.reverse();

    // Adding points to matrix
    for point in list {
        poly.add_col(&[point.0, point.1, point.2, 1.0]);
    }

    // Exiting function
    poly
}

/// Function that parses a vertex, vertex texture, or a vertex normal line
pub fn parse_three_float(line: &str) -> (f32, f32, f32) {
    // Variable declarations
    let mut temp: Vec<f32> = Vec::new();

    // Iterating through split line
    for (i, str) in (0_i32..).zip(line.split(' ')) {
        if (i > 0 && !str.is_empty()) {
            temp.push(str.parse::<f32>().unwrap_or(0.0));
        }
    }

    // Exiting function
    (temp[0_usize], temp[1_usize], temp[2_usize])
}
