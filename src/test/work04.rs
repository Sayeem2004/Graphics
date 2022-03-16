// Imports
use crate::format::file;
use crate::template::work04;

/// Testing out work04 stuff
#[test]
fn test_work04() {
    // Running function
    work04::create_work04_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/script/w04_face.png"));
    assert!(file::file_exists("image/ppm/w04_optical.ppm"));
    assert!(file::file_exists("image/script/w04_girl.png"));
}
