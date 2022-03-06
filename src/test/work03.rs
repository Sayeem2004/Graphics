// Imports
use crate::template::work03;
use crate::format::file;

/// Testing out work03 stuff
#[test]
fn test_work03() {
    // Running function
    work03::create_work03_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/usr/w03_pic.png"));
    assert!(file::file_exists("image/ppm/w03_tesseract.ppm"));
}
