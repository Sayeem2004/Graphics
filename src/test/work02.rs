// Imports
use crate::template::work02;
use crate::format::file;

/// Testing out work02 stuff
#[test]
fn test_work02() {
    // Running function
    work02::test_matrix();
    work02::create_work02_images();

    // Making sure image files exist
    assert!(file::file_exists("image/ppm/w02_matrix.ppm"));
    assert!(file::file_exists("image/ppm/w02_lotus.ppm"));
}
