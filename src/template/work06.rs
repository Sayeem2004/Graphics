// Imports
use crate::algorithm::shape;
use crate::format::{constant, file, image::Image, matrix::Matrix, pixel::Pixel};
use crate::script::parse;
use std::{fs, process::Command};

/// Function that adds a gojo character to an image
fn gojo(edge: &mut Matrix, curr: &mut Image) {
    // Variable declarations
    let mut corr: Matrix = Matrix::new_matrix();

    // Drawing gojo lines and adding correction lines
    edge.draw_lines_rc(curr, Pixel::new_value(25, 25, 25));
    corr.add_edge(666.0, 4.0, 0.0, 690.0, 6.0, 0.0);
    corr.add_edge(710.0, 7.0, 0.0, 716.0, 4.0, 0.0);
    corr.add_edge(486.0, 332.0, 0.0, 490.0, 330.0, 0.0);
    corr.add_edge(543.0, 295.0, 0.0, 546.0, 295.0, 0.0);
    corr.add_edge(693.0, 427.0, 0.0, 693.0, 423.0, 0.0);
    corr.add_edge(692.0, 459.0, 0.0, 692.0, 457.0, 0.0);
    corr.add_edge(698.0, 435.0, 0.0, 698.0, 432.0, 0.0);
    corr.add_edge(768.0, 279.0, 0.0, 770.0, 279.0, 0.0);
    corr.add_edge(719.0, 382.0, 0.0, 720.0, 380.0, 0.0);
    corr.add_edge(828.0, 10.0, 0.0, 828.0, 0.0, 0.0);
    corr.add_edge(862.0, 240.0, 0.0, 865.0, 238.0, 0.0);
    corr.add_edge(668.0, 515.0, 0.0, 663.0, 505.0, 0.0);
    corr.add_edge(736.0, 470.0, 0.0, 734.0, 467.0, 0.0);
    corr.add_edge(747.0, 474.0, 0.0, 747.0, 470.0, 0.0);
    corr.add_edge(756.0, 497.0, 0.0, 756.0, 493.0, 0.0);
    corr.add_edge(703.0, 540.0, 0.0, 700.0, 535.0, 0.0);
    corr.add_edge(621.0, 550.0, 0.0, 619.0, 555.0, 0.0);
    corr.draw_lines_xy(curr, Pixel::new_value(25, 25, 25));

    // Adding colors to gojo
    curr.flood_xy(
        660,
        115,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        712,
        130,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        725,
        130,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        666,
        10,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        712,
        7,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        700,
        220,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        936,
        320,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        812,
        300,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        900,
        285,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        745,
        360,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        445,
        300,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        445,
        300,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        695,
        445,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(80, 75, 79),
    );
    curr.flood_xy(
        672,
        340,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(251, 247, 246),
    );
    curr.flood_xy(
        691,
        315,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        710,
        370,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        690,
        355,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        719,
        380,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        719,
        380,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        700,
        175,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(25, 25, 25),
    );
    curr.flood_xy(
        666,
        435,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(25, 25, 25),
    );
    curr.flood_xy(
        735,
        530,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(108, 230, 241),
    );
    curr.flood_xy(
        700,
        500,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(247, 243, 244),
    );
    curr.flood_xy(
        670,
        465,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        742,
        470,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(165, 142, 148),
    );
    curr.flood_xy(
        652,
        490,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        750,
        483,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        683,
        620,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        745,
        560,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        660,
        545,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        790,
        556,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        700,
        545,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        625,
        560,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(215, 215, 215),
    );
    curr.flood_xy(
        715,
        585,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(227, 230, 235),
    );
    curr.flood_xy(
        655,
        505,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        695,
        520,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        645,
        515,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        650,
        508,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        665,
        510,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(180, 180, 180),
    );
    curr.flood_xy(
        635,
        480,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(1, 1, 1),
    );
    curr.flood_xy(
        770,
        490,
        Pixel::new_value(25, 25, 25),
        Pixel::new_value(1, 1, 1),
    );
}

/// Function that creates the purple hollow image
fn purple_hollow() {
    // Variable declarations
    let mut curr: Image = Image::new_dimension(1400, 800);
    let mut edge: Matrix = file::read_lines_csv("data/w06/w06_gojo.csv");
    let mut red: Matrix = Matrix::new_matrix();
    let mut blue: Matrix = Matrix::new_matrix();
    let mut trans_red: Matrix = Matrix::new_transformation();
    let mut trans_blue: Matrix = Matrix::new_transformation();

    // Attempting to create image directory
    fs::create_dir_all("image/tmp").expect("Unable to create image/tmp directory");
    fs::create_dir_all("image/gif").expect("Unable to create image/gif directory");

    // Making red sphere and red translation matrix
    shape::add_sphere(&mut red, 375.0, 400.0, 0.0, 1.0, 120);
    trans_red.translate(-375.0, -400.0, 0.0);
    trans_red.dilate(1.067, 1.067, 1.067);
    trans_red.translate(375.0, 400.0, 0.0);

    // Making blue sphere and blue translation matrix
    shape::add_sphere(&mut blue, 1025.0, 400.0, 0.0, 1.0, 120);
    trans_blue.translate(-1025.0, -400.0, 0.0);
    trans_blue.dilate(1.067, 1.067, 1.067);
    trans_blue.translate(1025.0, 400.0, 0.0);

    // Iterating through movements
    for i in 0..90 {
        // Drawing spheres
        curr.fill(constant::BLACK_PIXEL);
        red.draw_triangles_xy(&mut curr, Pixel::new_value(70 + i * 2, 0, 0));
        blue.draw_triangles_xy(&mut curr, Pixel::new_value(0, 0, 70 + i * 2));

        // Adding gojo image
        gojo(&mut edge, &mut curr);

        // Saving image
        file::create_ppm_ascii(
            &vec![
                "image/tmp/purple0".to_string(),
                format!("{:0>#3}", i.to_string()),
                ".ppm".to_string(),
            ]
            .join(""),
            &curr,
            0,
        );

        // Translating spheres
        red.matrix_transform(&trans_red);
        blue.matrix_transform(&trans_blue);
    }

    // Updating red transformation matrix
    trans_red = Matrix::new_transformation();
    trans_red.translate(-375.0, -400.0, 0.0);
    trans_red.rotate_degree(4.0, "y");
    trans_red.translate(375.0, 400.0, 0.0);

    // Updating blue transformation matrix
    trans_blue = Matrix::new_transformation();
    trans_blue.translate(-1025.0, -400.0, 0.0);
    trans_blue.rotate_degree(-4.0, "y");
    trans_blue.translate(1025.0, 400.0, 0.0);

    // Iterating through movements
    for q in 90..180 {
        // Drawing spheres
        curr.fill(constant::BLACK_PIXEL);
        red.draw_triangles_xy(&mut curr, Pixel::new_value(250, 0, 0));
        blue.draw_triangles_xy(&mut curr, Pixel::new_value(0, 0, 250));

        // Adding gojo image
        gojo(&mut edge, &mut curr);

        // Saving image
        file::create_ppm_ascii(
            &vec![
                "image/tmp/purple0".to_string(),
                format!("{:0>#3}", q.to_string()),
                ".ppm".to_string(),
            ]
            .join(""),
            &curr,
            0,
        );

        // Translating spheres
        red.matrix_transform(&trans_red);
        blue.matrix_transform(&trans_blue);
    }

    // Performing image magick convert command
    println!("Converting purple images to w06_purple_hollow.gif...");
    Command::new("convert")
        .arg("-delay")
        .arg("5")
        .arg("-loop")
        .arg("0")
        .arg("image/tmp/purple*.ppm")
        .arg("image/gif/w06_purple_hollow.gif")
        .status()
        .expect("Convert command failed to run");
    println!("Image file is named image/gif/w06_purple_hollow.gif");

    // Removing temp directory
    let res = fs::remove_dir_all("image/tmp");
    match res {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Unable to remove temporary image/tmp directory");
        }
    }
}

/// Function that creates all images from work 06
pub fn create_work06_images(mode: i32) {
    // Attempting to create image directory
    fs::create_dir_all("image/ppm").expect("Unable to create image/ppm directory");

    // Creating shapes image test
    parse::parse("data/w06/w06_script", 500, mode);

    // Creating face image
    parse::parse("data/w06/w06_face", 500, mode);

    // Creating purple hollow image
    purple_hollow();
}
