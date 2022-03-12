// Imports
use crate::format::file;
use crate::template::work02;

/// Testing out work02 stuff
#[test]
fn test_work02() {
    // Running function
    work02::create_work02_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/ppm/w02_matrix.ppm"));
    assert!(file::file_exists("image/ppm/w02_lotus.ppm"));
    assert!(file::file_exists("image/ppm/w02_rainbow_lotus.ppm"));
    assert!(file::file_exists("image/ppm/w02_eru.ppm"));
}
