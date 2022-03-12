// Imports
use crate::format::file;
use crate::template::work03;

/// Testing out work03 stuff
#[test]
fn test_work03() {
    // Running function
    work03::create_work03_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/script/w03_pic.png"));
    assert!(file::file_exists("image/ppm/w03_tesseract.ppm"));
    assert!(file::file_exists("image/script/w03_triangle.png"));
    assert!(file::file_exists("image/script/w03_dragon.png"));
}
