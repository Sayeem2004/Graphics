// Imports
use crate::format::{image::Image, pixel::Pixel, util};
use std::{
    fs,
    process::{Command, Output},
    str::Split,
};

/// Function that sends data to topo.py and generate random data using perlin noise
pub fn create_random_terrain(size: i32, freq: f32, water: f32, img: &mut Image) {
    // Running python generator on arguments
    let output: Output = Command::new("python")
        .arg("src/algorithm/topo.py")
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
    let result = fs::read_to_string(csv_path);
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
