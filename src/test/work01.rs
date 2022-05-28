// Imports
use crate::format::util;
use crate::template::work01;

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
