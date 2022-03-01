// Imports
use crate::template::work01;
use crate::format::file;

/// Testing out work00 stuff
#[test]
fn test_work01() {
    // Running function
    work01::create_work01_images();

    // Making sure image files exist
    assert!(file::file_exists("image/ppm/w01_lines.ppm"));
    assert!(file::file_exists("image/ppm/w01_sierpinski.ppm"));
    assert!(file::file_exists("image/ppm/w01_heighway.ppm"));
    assert!(file::file_exists("image/ppm/w01_bintree.ppm"));
    assert!(file::file_exists("image/ppm/w01_koch.ppm"));
}
