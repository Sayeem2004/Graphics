// Imports
use crate::template::prev::ver06::algorithm::shape;
use crate::template::prev::ver06::format::{constant, file, image::Image, matrix::Matrix, pixel::Pixel};
use crate::template::prev::ver06::script::parse;
use std::{fs, process::Command};

/// Function that adds a gojo character to an image
fn gojo(edge: &mut Matrix, curr: &mut Image) {
    // Variable declarations
    let mut corr: Matrix = Matrix::new_matrix();

    // Drawing gojo lines and adding correction lines
    edge.draw_lines_rc(curr, Pixel::new_value(25, 25, 25));
    corr.add_edge((333.0, 2.0, 0.0), (345.0, 3.0, 0.0));
    corr.add_edge((355.0, 3.0, 0.0), (358.0, 2.0, 0.0));
    corr.add_edge((243.0, 166.0, 0.0), (245.0, 165.0, 0.0));
    corr.add_edge((271.0, 147.0, 0.0), (273.0, 147.0, 0.0));
    corr.add_edge((346.0, 213.0, 0.0), (346.0, 211.0, 0.0));
    corr.add_edge((346.0, 229.0, 0.0), (346.0, 228.0, 0.0));
    corr.add_edge((349.0, 217.0, 0.0), (349.0, 216.0, 0.0));
    corr.add_edge((384.0, 139.0, 0.0), (385.0, 139.0, 0.0));
    corr.add_edge((359.0, 191.0, 0.0), (360.0, 190.0, 0.0));
    corr.add_edge((414.0, 5.0, 0.0), (414.0, 0.0, 0.0));
    corr.add_edge((431.0, 120.0, 0.0), (432.0, 119.0, 0.0));
    corr.add_edge((334.0, 257.0, 0.0), (331.0, 252.0, 0.0));
    corr.add_edge((368.0, 235.0, 0.0), (367.0, 233.0, 0.0));
    corr.add_edge((373.0, 237.0, 0.0), (373.0, 235.0, 0.0));
    corr.add_edge((378.0, 248.0, 0.0), (378.0, 246.0, 0.0));
    corr.add_edge((351.0, 270.0, 0.0), (350.0, 267.0, 0.0));
    corr.add_edge((310.0, 275.0, 0.0), (309.0, 277.0, 0.0));
    corr.draw_lines_xy(curr, Pixel::new_value(25, 25, 25));

    // Adding colors to gojo
    curr.flood_xy(
        330,
        57,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        280,
        210,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        355,
        220,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(25, 25, 25),
    );
    curr.flood_xy(
        356,
        65,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        362,
        65,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        333,
        5,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        356,
        3,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        350,
        110,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        468,
        160,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        406,
        150,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        450,
        142,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        372,
        180,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        224,
        150,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        222,
        150,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        347,
        222,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        336,
        170,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(251, 247, 246),
    );
    curr.flood_xy(
        345,
        157,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        355,
        185,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        345,
        177,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        359,
        190,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        359,
        190,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        350,
        87,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(25, 25, 25),
    );
    curr.flood_xy(
        333,
        217,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(25, 25, 25),
    );
    curr.flood_xy(
        367,
        265,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(108, 230, 241),
    );
    curr.flood_xy(
        350,
        250,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(247, 243, 244),
    );
    curr.flood_xy(
        335,
        232,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        371,
        235,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        326,
        245,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        375,
        241,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        341,
        310,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        372,
        280,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        330,
        272,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        395,
        278,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        350,
        272,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        312,
        280,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        357,
        292,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(227, 230, 235),
    );
    curr.flood_xy(
        327,
        252,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        347,
        260,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        322,
        257,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        325,
        254,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        332,
        255,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        317,
        240,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(1, 1, 1),
    );
    curr.flood_xy(
        385,
        245,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(1, 1, 1),
    );
}

/// Function that makes a changing perspective gif
fn perspectives() {
    // Variable declarations
    let mut names: Vec<String> = Vec::new();
    let mut curr: Image = Image::new_dimension(750, 750);
    let mut sphere: Matrix = Matrix::new_matrix();
    shape::add_sphere(&mut sphere, (375.0, 375.0, 0.0), 300.0, 100);

    // Attempting to create image directory
    fs::create_dir_all("temp").expect("Unable to create temp directory");
    fs::create_dir_all("image/gif").expect("Unable to create image/gif directory");

    // Iterating through perspectives
    for i in 0..=100 {
        // Drawing sphere
        sphere.draw_triangles_xy(
            &mut curr,
            constant::WHITE_PIXEL,
            &[100.0 - 2.0 * (i as f32), 0.0, -1.0],
        );

        // Saving image
        let name: String = vec![
            "temp/perspective".to_string(),
            format!("{:0>#3}", i as i32),
            ".ppm".to_string(),
        ]
        .join("");
        file::create_ppm_ascii(&name, &curr, 0);
        names.push(name);
    }

    // Performing image magick convert command
    println!("Converting perspective images to w06_perspectives.gif...");
    Command::new("convert")
        .arg("-delay")
        .arg("1.7")
        .arg("-loop")
        .arg("0")
        .arg("temp/perspective*.ppm")
        .arg("image/gif/w06_perspectives.gif")
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named image/gif/w06_perspectives.gif");

    // Removing temporary files
    for file in names.iter() {
        fs::remove_file(&file).expect("Unable to delete temporary gif file");
    }
}

/// Function that creates the purple hollow image
fn purple_hollow() {
    // Variable declarations
    let mut curr: Image = Image::new_dimension(700, 400);
    let mut edge: Matrix = file::read_lines_csv("data/csv/w06_gojo.csv");
    let mut red: Matrix = Matrix::new_matrix();
    let mut blue: Matrix = Matrix::new_matrix();
    let mut trans: Matrix = Matrix::new_transformation();
    let mut trans_red: Matrix = Matrix::new_transformation();
    let mut trans_blue: Matrix = Matrix::new_transformation();
    let mut names: Vec<String> = Vec::new();
    trans.dilate(0.5, 0.5, 0.5);
    edge.matrix_transform(&trans);

    // Attempting to create image directory
    fs::create_dir_all("temp").expect("Unable to create temp directory");
    fs::create_dir_all("image/gif").expect("Unable to create image/gif directory");

    // Making red sphere and red translation matrix
    shape::add_sphere(&mut red, (187.0, 200.0, 0.0), 1.0, 120);
    trans_red.translate(-187.0, -200.0, 0.0);
    trans_red.dilate(1.059, 1.059, 1.059);
    trans_red.translate(187.0, 200.0, 0.0);

    // Making blue sphere and blue translation matrix
    shape::add_sphere(&mut blue, (512.0, 200.0, 0.0), 1.0, 120);
    trans_blue.translate(-512.0, -200.0, 0.0);
    trans_blue.dilate(1.059, 1.059, 1.059);
    trans_blue.translate(512.0, 200.0, 0.0);

    // Iterating through movements
    for i in 0..90 {
        // Drawing spheres
        curr.fill(constant::BLACK_PIXEL);
        red.draw_triangles_xy(
            &mut curr,
            Pixel::new_value(70 + i * 2, 0, 0),
            &constant::ZVIEW,
        );
        blue.draw_triangles_xy(
            &mut curr,
            Pixel::new_value(0, 0, 70 + i * 2),
            &constant::ZVIEW,
        );

        // Adding gojo image
        gojo(&mut edge, &mut curr);

        // Saving image
        let name: String = vec![
            "temp/purple".to_string(),
            format!("{:0>#3}", i),
            ".ppm".to_string(),
        ]
        .join("");
        file::create_ppm_ascii(&name, &curr, 0);
        names.push(name);

        // Translating spheres
        red.matrix_transform(&trans_red);
        blue.matrix_transform(&trans_blue);
    }

    // Updating red transformation matrix
    trans_red = Matrix::new_transformation();
    trans_red.translate(-187.0, -200.0, 0.0);
    trans_red.rotate_degree(4.0, "y");
    trans_red.translate(187.0, 200.0, 0.0);

    // Updating blue transformation matrix
    trans_blue = Matrix::new_transformation();
    trans_blue.translate(-512.0, -200.0, 0.0);
    trans_blue.rotate_degree(-4.0, "y");
    trans_blue.translate(512.0, 200.0, 0.0);

    // Iterating through movements
    for q in 90..180 {
        // Drawing spheres
        curr.fill(constant::BLACK_PIXEL);
        red.draw_triangles_xy(&mut curr, Pixel::new_value(250, 0, 0), &constant::ZVIEW);
        blue.draw_triangles_xy(&mut curr, Pixel::new_value(0, 0, 250), &constant::ZVIEW);

        // Adding gojo image
        gojo(&mut edge, &mut curr);

        // Saving image
        let name: String = vec![
            "temp/purple".to_string(),
            format!("{:0>#3}", q),
            ".ppm".to_string(),
        ]
        .join("");
        file::create_ppm_ascii(&name, &curr, 0);
        names.push(name);

        // Translating spheres
        red.matrix_transform(&trans_red);
        blue.matrix_transform(&trans_blue);
    }

    // Performing image magick convert command
    println!("Converting purple images to w06_purple_hollow.gif...");
    Command::new("convert")
        .arg("-delay")
        .arg("1.7")
        .arg("-loop")
        .arg("0")
        .arg("temp/purple*.ppm")
        .arg("image/gif/w06_purple_hollow.gif")
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named image/gif/w06_purple_hollow.gif");

    // Removing temporary files
    for file in names.iter() {
        fs::remove_file(&file).expect("Unable to delete temporary gif file");
    }
}

/// Function that creates all images from work 06
pub fn create_work06_images(mode: i32) {
    // Creating shapes image test
    parse::parse("data/mdl/w06_script.mdl", 750, mode);

    // Creating face image
    parse::parse("data/mdl/w06_face.mdl", 750, mode);

    // Creating purple hollow image
    purple_hollow();

    // Creating perspectives image
    perspectives();

    // Creating sphere of spheres image
    parse::parse("data/mdl/w06_spheres.mdl", 750, mode);
}
