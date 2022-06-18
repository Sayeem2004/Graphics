// Imports
use crate::format::util;
use crate::template::work00;
use crate::template::work01;
use crate::template::work02;
use crate::template::work03;
use crate::template::work04;
use crate::template::work05;
use crate::template::work06;
use crate::template::work07;
use crate::template::work08;
use crate::template::work09;
use crate::template::work10;
use crate::template::work11;
use crate::template::work12;

/// Testing out work00 stuff
#[test]
fn test_work00() {
    // Running function
    work00::create_work00_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/ppm/w00_corridor.ppm"));
    assert!(util::file_exists("image/ppm/w00_checkerboard.ppm"));
    assert!(util::file_exists("image/ppm/w00_chains.ppm"));
    assert!(util::file_exists("image/ppm/w00_barnsley.ppm"));
}

/// Testing out work01 stuff
#[test]
fn test_work01() {
    // Running function
    work01::create_work01_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/ppm/w01_lines.ppm"));
    assert!(util::file_exists("image/ppm/w01_sierpinski.ppm"));
    assert!(util::file_exists("image/ppm/w01_heighway.ppm"));
    assert!(util::file_exists("image/ppm/w01_bintree.ppm"));
    assert!(util::file_exists("image/ppm/w01_koch.ppm"));
}

/// Testing out work02 stuff
#[test]
fn test_work02() {
    // Running function
    work02::create_work02_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/ppm/w02_matrix.ppm"));
    assert!(util::file_exists("image/ppm/w02_lotus.ppm"));
    assert!(util::file_exists("image/ppm/w02_rainbow_lotus.ppm"));
    assert!(util::file_exists("image/ppm/w02_eru.ppm"));
}

/// Testing out work03 stuff
#[test]
fn test_work03() {
    // Running function
    work03::create_work03_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w03_pic.png"));
    assert!(util::file_exists("image/ppm/w03_tesseract.ppm"));
    assert!(util::file_exists("image/script/w03_triangle.png"));
    assert!(util::file_exists("image/script/w03_dragon.png"));
}

/// Testing out work04 stuff
#[test]
fn test_work04() {
    // Running function
    work04::create_work04_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w04_face.png"));
    assert!(util::file_exists("image/ppm/w04_optical.ppm"));
    assert!(util::file_exists("image/script/w04_girl.png"));
}

/// Testing out work05 stuff
#[test]
fn test_work05() {
    // Running function
    work05::create_work05_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w05_face.png"));
    assert!(util::file_exists("image/script/w05_solar.png"));
    assert!(util::file_exists("image/ppm/w05_filled_sphere.ppm"));
    assert!(util::file_exists("image/ppm/w05_rgb_sphere.ppm"));
}

/// Testing out work06 stuff
#[test]
fn test_work06() {
    // Running function
    work06::create_work06_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w06_face.png"));
    assert!(util::file_exists("image/gif/w06_purple_hollow.gif"));
    assert!(util::file_exists("image/gif/w06_perspectives.gif"));
    assert!(util::file_exists("image/script/w06_spheres.png"));
}

/// Testing out work07 stuff
#[test]
fn test_work07() {
    // Running function
    work07::create_work07_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w07_robot.png"));
    assert!(util::file_exists("image/ppm/w07_spiralchain.ppm"));
    assert!(util::file_exists("image/ppm/w07_olympics.ppm"));
    assert!(util::file_exists("image/script/w07_umbrella.png"));
}

/// Testing out work08 stuff
#[test]
fn test_work08() {
    // Running function
    work08::create_work08_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w08_robot.png"));
    assert!(util::file_exists("image/ppm/w08_gradient.ppm"));
    assert!(util::file_exists("image/gif/w08_night_sky.gif"));
    assert!(util::file_exists("image/ppm/w08_color_wheel.ppm"));
}

/// Testing out work09 stuff
#[test]
fn test_work09() {
    // Running function
    work09::create_work09_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w09_face.png"));
    assert!(util::file_exists("image/script/w09_robot.png"));
    assert!(util::file_exists("image/gif/w09_rotating_slab.gif"));
}

/// Testing out work10 stuff
#[test]
fn test_work10() {
    // Running function
    work10::create_work10_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w10_face.png"));
    assert!(util::file_exists("image/script/w10_test.png"));
    assert!(util::file_exists("image/script/w10_excalibur.png"));
    assert!(util::file_exists("image/script/w10_trophy.png"));
    assert!(util::file_exists("image/script/w10_W.png"));
}

/// Testing out work11 stuff
#[test]
fn test_work11() {
    // Running function
    work11::create_work11_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/gif/w11_simple_300.gif"));
    assert!(util::file_exists("image/gif/w11_spiral.gif"));
    assert!(util::file_exists("image/gif/w11_wave.gif"));
    assert!(util::file_exists("image/gif/w11_trophy.gif"));
}

/// Testing out work12 stuff
#[test]
fn test_work12() {
    // Running function
    work12::create_work12_images(1);

    // Making sure image files exist
    assert!(util::file_exists("image/script/w12_savecs_test.png"));
    assert!(util::file_exists("image/gif/w12_lighting_test.gif"));
    assert!(util::file_exists("image/gif/w12_knobs_test.gif"));
    assert!(util::file_exists("image/script/w12_terrain.png"));
}
