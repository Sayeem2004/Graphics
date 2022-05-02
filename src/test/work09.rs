// Imports
use crate::format::file;
use crate::template::work09;

/// Testing out work08 stuff
#[test]
fn test_work09() {
    // Running function
    work09::create_work09_images(1);

    // Making sure image files exist
    assert!(file::file_exists("image/script/w09_face.png"));
    assert!(file::file_exists("image/script/w09_robot.png"));
}
